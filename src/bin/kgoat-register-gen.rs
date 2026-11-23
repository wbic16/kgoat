//! kgoat-register-gen — KGOAT Stage 2, registers leg (SPEC §6 Stage 2,
//! §4.1; DESIGN.md Addendum 6). Reads one of the four registers and
//! writes a working copy carrying the graph layer in volume 1.3.1.
//! Production phexts are never modified.
//!
//!   kgoat-register-gen artifacts <phext> <artifacts-source.json> <out> [hop]
//!   kgoat-register-gen tasks     <phext> <tasks-source.json>     <out> [hop]
//!   kgoat-register-gen skills    <phext> <out> [hop]
//!   kgoat-register-gen state     <phext> <out> [hop]
//!
//! Every mode gates before anything is written: the register's own
//! records must reconcile with the phext's scrolls (counts, presence,
//! and — where the register carries them — its sha256 receipts).
//! Placement mirrors the ietf/research legs: register work node
//! (graph Home) at 1.1.1/1.3.1/1.1.1, content nodes at
//! 1.1.1/1.3.1/1.2.<ordinal>, every content node part_of the work
//! node at w0 with the unit's source coordinate as evidence.
//! No claim nodes: none of these registers carries a MOAT assay
//! layer, and no assay is fabricated (§4.3 governs claim-bearing
//! material; works and events are not claims).

use std::collections::{HashMap, HashSet};
use std::env;
use std::process::ExitCode;

use libphext::phext;
use serde::Deserialize;
use sha2::{Digest, Sha256};

const HOME: &str = "1.1.1/1.3.1/1.1.1";
const CONTENT_PREFIX: &str = "1.1.1/1.3.1/1.2.";
const PHEXT_HOME: &str = "1.1.1/1.1.1/1.1.1";

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Node-scroll lines are single lines: collapse all whitespace runs
/// (including newlines) to single spaces. Returns (text, changed).
fn one_line(s: &str) -> (String, bool) {
    let out: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
    (out.clone(), out != s)
}

/// The phext's non-empty scrolls, in explode (file) order.
fn nonempty(phext_text: &str) -> Vec<(String, String)> {
    phext::explode(phext_text)
        .iter()
        .filter(|(_, s)| !s.trim().is_empty())
        .map(|(c, s)| (c.to_string(), s.clone()))
        .collect()
}

struct Unit {
    kind: &'static str,
    statement: String,
    body: Vec<String>,
    source_coord: String,
}

struct GenOut {
    text: String,
    nodes: usize,
    report: String,
}

/// Assemble the working copy: original scrolls untouched, graph
/// Home + one content node per unit added in volume 1.3.1.
fn assemble(phext_text: &str, home_text: String, units: &[Unit], hop: &str) -> GenOut {
    let mut out: HashMap<phext::Coordinate, String> = phext::explode(phext_text)
        .iter()
        .map(|(c, s)| (c.clone(), s.clone()))
        .collect();
    for (i, u) in units.iter().enumerate() {
        let mut node = String::new();
        node.push_str(&format!("node · {} · {}\n", u.kind, u.statement));
        for line in &u.body {
            node.push_str(line);
            node.push('\n');
        }
        node.push_str(&format!(
            "edge · part_of · {HOME} · w0 · {} · {hop}\n",
            u.source_coord
        ));
        out.insert(
            phext::to_coordinate(&format!("{CONTENT_PREFIX}{}", i + 1)),
            node,
        );
    }
    out.insert(phext::to_coordinate(HOME), home_text);
    GenOut {
        text: phext::implode(out),
        nodes: units.len() + 1,
        report: String::new(),
    }
}

/// Fidelity: every original scroll byte-identical in the copy.
fn fidelity_ok(before_text: &str, after_text: &str) -> bool {
    let before: HashMap<String, String> = phext::explode(before_text)
        .iter()
        .map(|(c, s)| (c.to_string(), s.clone()))
        .collect();
    let after = phext::explode(after_text);
    if after.len() < before.len() {
        return false;
    }
    for (c, s) in &before {
        match after.iter().find(|(ac, _)| &ac.to_string() == c) {
            Some((_, as_)) if as_ == s => {}
            _ => return false,
        }
    }
    true
}

// ---------- artifacts ----------

#[derive(Debug, Deserialize)]
struct ArtifactsSource {
    artifacts: Vec<ArtifactEntry>,
}

#[derive(Debug, Deserialize)]
struct ArtifactEntry {
    coordinate: String,
    inscribed: String,
    sha256: String,
    source: String,
    title: String,
}

fn gen_artifacts(
    phext_text: &str,
    src_text: &str,
    phext_sha: &str,
    hop: &str,
) -> Result<GenOut, String> {
    let src: ArtifactsSource =
        serde_json::from_str(src_text).map_err(|e| format!("cannot parse artifacts source: {e}"))?;
    let scrolls = nonempty(phext_text);
    let by_coord: HashMap<String, String> = scrolls.iter().cloned().collect();
    if !by_coord.contains_key(PHEXT_HOME) {
        return Err("artifacts phext has no Home scroll".into());
    }
    if src.artifacts.len() + 1 != scrolls.len() {
        return Err(format!(
            "count mismatch: {} source entries + Home != {} non-empty scrolls",
            src.artifacts.len(),
            scrolls.len()
        ));
    }
    let mut adjusted = 0usize;
    let mut units: Vec<Unit> = Vec::new();
    for a in &src.artifacts {
        let scroll = by_coord
            .get(&a.coordinate)
            .ok_or_else(|| format!("no scroll at source coordinate {}", a.coordinate))?;
        // The scroll's own provenance header is the receipt: its
        // SHA-256 and Title lines must equal the source record.
        let hdr_sha = scroll
            .lines()
            .find_map(|l| l.strip_prefix("SHA-256:"))
            .map(|s| s.trim().to_string())
            .ok_or_else(|| format!("scroll {} has no SHA-256 header", a.coordinate))?;
        if hdr_sha != a.sha256 {
            return Err(format!(
                "receipt mismatch at {}: scroll header {hdr_sha} != source {}",
                a.coordinate, a.sha256
            ));
        }
        let hdr_title = scroll
            .lines()
            .find_map(|l| l.strip_prefix("Title:"))
            .map(|s| s.trim().to_string())
            .ok_or_else(|| format!("scroll {} has no Title header", a.coordinate))?;
        if hdr_title != a.title.trim() {
            return Err(format!(
                "title mismatch at {}: scroll {hdr_title:?} != source {:?}",
                a.coordinate, a.title
            ));
        }
        let (title, c) = one_line(&a.title);
        adjusted += c as usize;
        units.push(Unit {
            kind: "work",
            statement: title,
            body: vec![
                format!("source · artifacts@{}", a.coordinate),
                format!("inscribed · {}", one_line(&a.inscribed).0),
                format!("sha256 · {}", a.sha256),
                format!("source-path · {}", one_line(&a.source).0),
            ],
            source_coord: a.coordinate.clone(),
        });
    }
    let home = format!(
        "node · work · artifacts.phext — the register of generated artifacts, as a phext · KGOAT graph layer home\n\
         The graph layer of artifacts.phext (SPEC §6 Stage 2): {} artifacts, each a work node (1.2.<ordinal>, ordinal = artifacts-source.json order) carrying its inscription receipt — the sha256 of the artifact's source file, cross-checked against the scroll's own provenance header before generation.\n\
         source phext sha256: {phext_sha} · units: {} · kinds: work {}\n\
         No claim nodes: this register carries no MOAT assay layer, and no assay is fabricated for a work.\n\
         Generated by kgoat-register-gen (artifacts) · hop {hop} · the scrolls are right; this layer is a reading of them (§2.3).\n",
        units.len(),
        units.len(),
        units.len()
    );
    let mut g = assemble(phext_text, home, &units, hop);
    g.report = format!("{} fields adjusted (line-collapsed)", adjusted);
    Ok(g)
}

// ---------- tasks ----------

#[derive(Debug, Deserialize)]
struct TasksSource {
    tasks: Vec<TaskEntry>,
}

#[derive(Debug, Deserialize)]
struct TaskEntry {
    coordinate: String,
    issued: String,
    title: String,
    status: String,
}

fn gen_tasks(
    phext_text: &str,
    src_text: &str,
    phext_sha: &str,
    hop: &str,
) -> Result<GenOut, String> {
    let src: TasksSource =
        serde_json::from_str(src_text).map_err(|e| format!("cannot parse tasks source: {e}"))?;
    let scrolls = nonempty(phext_text);
    let by_coord: HashMap<String, String> = scrolls.iter().cloned().collect();
    if !by_coord.contains_key(PHEXT_HOME) {
        return Err("tasks phext has no Home scroll".into());
    }
    if src.tasks.len() + 1 != scrolls.len() {
        return Err(format!(
            "count mismatch: {} source records + Home != {} non-empty scrolls",
            src.tasks.len(),
            scrolls.len()
        ));
    }
    let mut adjusted = 0usize;
    let mut units: Vec<Unit> = Vec::new();
    for t in &src.tasks {
        if !by_coord.contains_key(&t.coordinate) {
            return Err(format!("no scroll at source coordinate {}", t.coordinate));
        }
        let (title, c1) = one_line(&t.title);
        let (status, c2) = one_line(&t.status);
        adjusted += c1 as usize + c2 as usize;
        units.push(Unit {
            kind: "work",
            statement: title,
            body: vec![
                format!("source · tasks@{}", t.coordinate),
                format!("issued · {}", one_line(&t.issued).0),
                format!("status-record · {status}"),
            ],
            source_coord: t.coordinate.clone(),
        });
    }
    let home = format!(
        "node · work · tasks.phext — Will's task register, works in flight · KGOAT graph layer home\n\
         The graph layer of tasks.phext (SPEC §6 Stage 2): {} tasks, each a work node (1.2.<ordinal>, ordinal = tasks-source.json order). A task's workflow status rides as a status-record provenance line — it is the register's own record, not a MOAT assay, and is never written as one.\n\
         source phext sha256: {phext_sha} · units: {} · kinds: work {}\n\
         No claim nodes: this register carries no MOAT assay layer, and no assay is fabricated for a work.\n\
         Generated by kgoat-register-gen (tasks) · hop {hop} · the scrolls are right; this layer is a reading of them (§2.3).\n",
        units.len(),
        units.len(),
        units.len()
    );
    let mut g = assemble(phext_text, home, &units, hop);
    g.report = format!("{} fields adjusted (line-collapsed)", adjusted);
    Ok(g)
}

// ---------- skills ----------

const SHELF_LABELS: [&str; 14] = [
    "Home/meta",
    "Verification",
    "Phext-native language",
    "Operations",
    "Onboarding",
    "Film",
    "Memory & record",
    "Work management",
    "Research",
    "Coordination",
    "Upkeep & presence",
    "Songwriting",
    "Legacy stack",
    "Personas",
];

struct IndexEntry {
    coord: String,
    name: String,
    file: String,
    sha_prefix: String,
}

/// Parse the skills Home index: lines of the form
/// `<coord>  <skill>  <file>  <n> bytes  sha256:<hex prefix>`,
/// after the "Index (coordinate" header line.
fn parse_index(home: &str) -> Result<Vec<IndexEntry>, String> {
    let mut in_index = false;
    let mut entries = Vec::new();
    for line in home.lines() {
        if line.starts_with("Index (coordinate") {
            in_index = true;
            continue;
        }
        if !in_index {
            continue;
        }
        let fields: Vec<&str> = line.split("  ").filter(|f| !f.is_empty()).collect();
        if fields.len() < 5 {
            continue;
        }
        if fields[1] == "(this scroll)" {
            continue;
        }
        let sha = fields[4]
            .strip_prefix("sha256:")
            .ok_or_else(|| format!("index line lacks sha256: {line}"))?;
        entries.push(IndexEntry {
            coord: fields[0].trim().to_string(),
            name: fields[1].trim().to_string(),
            file: fields[2].trim().to_string(),
            sha_prefix: sha.trim().to_string(),
        });
    }
    if entries.is_empty() {
        return Err("skills Home index parsed to zero entries".into());
    }
    Ok(entries)
}

/// The skill key: (first group, second group) of a file's coordinate
/// — library.shelf.series / collection.volume.book; files of one
/// skill share both and differ in the third group.
fn skill_key(coord: &str) -> (String, String) {
    let groups: Vec<&str> = coord.split('/').collect();
    (
        groups.first().copied().unwrap_or("").to_string(),
        groups.get(1).copied().unwrap_or("").to_string(),
    )
}

fn shelf_of(coord: &str) -> Option<usize> {
    coord
        .split('/')
        .next()?
        .split('.')
        .nth(1)?
        .parse::<usize>()
        .ok()
}

fn skill_statement(scrolls: &HashMap<String, String>, skillmd: &str) -> String {
    let text = &scrolls[skillmd];
    let lines: Vec<&str> = text.lines().collect();
    // Frontmatter description (single line), if present.
    for l in lines.iter().take(12) {
        if let Some(d) = l.strip_prefix("description:") {
            let d = d.trim();
            if !d.is_empty() {
                return one_line(d).0;
            }
        }
    }
    for l in &lines {
        if let Some(h) = l.strip_prefix("# ") {
            return one_line(h).0;
        }
    }
    for l in &lines {
        let t = l.trim();
        if !t.is_empty() && t != "---" {
            return one_line(t).0;
        }
    }
    "skill".to_string()
}

fn gen_skills(phext_text: &str, phext_sha: &str, hop: &str) -> Result<GenOut, String> {
    let scrolls = nonempty(phext_text);
    let by_coord: HashMap<String, String> = scrolls.iter().cloned().collect();
    let home_text = by_coord
        .get(PHEXT_HOME)
        .ok_or("skills phext has no Home scroll")?;
    let entries = parse_index(home_text)?;
    // Gate: index entries == the non-Home scroll set, by coordinate.
    let indexed: HashSet<String> = entries.iter().map(|e| e.coord.clone()).collect();
    let present: HashSet<String> = by_coord
        .keys()
        .filter(|c| c.as_str() != PHEXT_HOME)
        .cloned()
        .collect();
    if indexed != present {
        return Err(format!(
            "index is not total over this phext: {} scrolls unindexed, {} entries without scrolls",
            present.difference(&indexed).count(),
            indexed.difference(&present).count()
        ));
    }
    // Gate: each entry's printed sha is a prefix of the sha256 of
    // the scroll's text (the scroll is the source file verbatim).
    for e in &entries {
        let full = sha256_hex(by_coord[&e.coord].as_bytes());
        if !full.starts_with(&e.sha_prefix) {
            return Err(format!(
                "receipt mismatch at {}: index sha {}… is not a prefix of computed {full}",
                e.coord, e.sha_prefix
            ));
        }
    }
    // Group files into skills, first-appearance order in the index.
    let mut order: Vec<(String, String)> = Vec::new();
    let mut groups: HashMap<(String, String), Vec<&IndexEntry>> = HashMap::new();
    for e in &entries {
        let key = skill_key(&e.coord);
        if !groups.contains_key(&key) {
            order.push(key.clone());
        }
        groups.entry(key).or_default().push(e);
    }
    let mut units: Vec<Unit> = Vec::new();
    let mut file_total = 0usize;
    for key in &order {
        let files = &groups[key];
        file_total += files.len();
        let name = files[0].name.clone();
        if files.iter().any(|f| f.name != name) {
            return Err(format!("skill at {key:?} carries two names in the index"));
        }
        let skillmd = files
            .iter()
            .find(|f| f.file == "SKILL.md")
            .map(|f| f.coord.clone())
            .ok_or_else(|| format!("skill {name} has no SKILL.md in the index"))?;
        let shelf = shelf_of(&skillmd).ok_or("bad shelf in coordinate")?;
        let label = SHELF_LABELS
            .get(shelf.wrapping_sub(1))
            .copied()
            .unwrap_or("unnamed shelf");
        let mut body = vec![
            format!("source · skills@{skillmd}"),
            format!("skill-name · {name}"),
            format!("shelf · {shelf} {label}"),
        ];
        for f in files {
            let full = sha256_hex(by_coord[&f.coord].as_bytes());
            body.push(format!("file · skills@{} · {} · sha256 {full}", f.coord, f.file));
        }
        units.push(Unit {
            kind: "work",
            statement: skill_statement(&by_coord, &skillmd),
            body,
            source_coord: skillmd,
        });
    }
    let home = format!(
        "node · work · skills.phext — Emise's skill register, capabilities as works · KGOAT graph layer home\n\
         The graph layer of skills.phext (SPEC §6 Stage 2): {} skills, each a work node (1.2.<ordinal>, ordinal = first appearance in the register's Home index). The phext stores one scroll per file ({} files); the node is the skill, and its body lists every file with its coordinate and full sha256 — each cross-checked against the index's printed receipt before generation. Persona skills are capabilities like any other: the node is the skill work, never a person (§5.2).\n\
         source phext sha256: {phext_sha} · units: {} · kinds: work {}\n\
         No claim nodes: this register carries no MOAT assay layer, and no assay is fabricated for a work.\n\
         Generated by kgoat-register-gen (skills) · hop {hop} · the scrolls are right; this layer is a reading of them (§2.3).\n",
        units.len(),
        file_total,
        units.len(),
        units.len()
    );
    let mut g = assemble(phext_text, home, &units, hop);
    g.report = format!("{file_total} files across {} skills", units.len());
    Ok(g)
}

// ---------- state ----------

/// One state line: "<timestamp> | <summary>", or the Home-collision
/// displaced form "<coordinate> | <timestamp> | <summary>".
fn parse_state_line(line: &str) -> (String, String) {
    let parts: Vec<&str> = line.split(" | ").collect();
    if parts.len() >= 3 && parts[0].contains('/') && !parts[0].contains(' ') {
        (parts[1].to_string(), parts[2..].join(" | "))
    } else if parts.len() >= 2 {
        (parts[0].to_string(), parts[1..].join(" | "))
    } else {
        (String::new(), line.to_string())
    }
}

fn gen_state(phext_text: &str, phext_sha: &str, hop: &str) -> Result<GenOut, String> {
    let scrolls = nonempty(phext_text);
    let mut adjusted = 0usize;
    let mut occurrences_total = 0usize;
    let mut units: Vec<Unit> = Vec::new();
    for (coord, text) in &scrolls {
        if coord == PHEXT_HOME {
            continue;
        }
        let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
        if lines.is_empty() {
            continue;
        }
        for l in &lines {
            if !l.contains(" | ") {
                return Err(format!("state scroll {coord} has a line without ' | ': {l}"));
            }
        }
        occurrences_total += lines.len();
        let (first_ts, first_summary) = parse_state_line(lines[0]);
        let (last_ts, _) = parse_state_line(lines[lines.len() - 1]);
        let (statement, c) = one_line(&first_summary);
        adjusted += c as usize;
        units.push(Unit {
            kind: "event",
            statement,
            body: vec![
                format!("source · state@{coord}"),
                format!("occurrences · {}", lines.len()),
                format!("first-seen · {first_ts}"),
                format!("last-seen · {last_ts}"),
            ],
            source_coord: coord.clone(),
        });
    }
    let home = format!(
        "node · work · state.phext — the ranch, distilled · KGOAT graph layer home\n\
         The graph layer of state.phext (SPEC §6 Stage 2): {} distilled ranch states, each an event node (1.2.<ordinal>, ordinal = the register's own order). A state scroll is one recurring state of the fleet; its lines are its occurrences ({occurrences_total} in this snapshot), and the node's statement is the first line's summary — the numbers inside a summary stay inside the statement text, as the register's own design carries them; this layer invents no metric nodes the register does not carry.\n\
         source phext sha256: {phext_sha} · units: {} · kinds: event {}\n\
         No claim nodes: this register carries no MOAT assay layer, and no assay is fabricated for an event.\n\
         Generated by kgoat-register-gen (state) · hop {hop} · the scrolls are right; this layer is a reading of them (§2.3).\n",
        units.len(),
        units.len(),
        units.len()
    );
    let mut g = assemble(phext_text, home, &units, hop);
    g.report = format!(
        "{occurrences_total} occurrences · {} fields adjusted (line-collapsed)",
        adjusted
    );
    Ok(g)
}

// ---------- main ----------

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let mode = args.first().map(|s| s.as_str()).unwrap_or("");
    let (phext_path, src_path, out_path, hop): (String, Option<String>, String, String) =
        match mode {
            "artifacts" | "tasks" if args.len() == 4 || args.len() == 5 => (
                args[1].clone(),
                Some(args[2].clone()),
                args[3].clone(),
                args.get(4).cloned().unwrap_or_else(|| "2026-11-23".into()),
            ),
            "skills" | "state" if args.len() == 3 || args.len() == 4 => (
                args[1].clone(),
                None,
                args[2].clone(),
                args.get(3).cloned().unwrap_or_else(|| "2026-11-23".into()),
            ),
            _ => {
                eprintln!("usage: kgoat-register-gen <artifacts|tasks> <phext> <source.json> <out> [hop]");
                eprintln!("       kgoat-register-gen <skills|state> <phext> <out> [hop]");
                return ExitCode::from(2);
            }
        };
    let phext_bytes = match std::fs::read(&phext_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("kgoat-register-gen: cannot read {phext_path}: {e}");
            return ExitCode::from(2);
        }
    };
    let phext_sha = sha256_hex(&phext_bytes);
    let phext_text = String::from_utf8_lossy(&phext_bytes).into_owned();
    let src_text = match &src_path {
        Some(p) => match std::fs::read_to_string(p) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("kgoat-register-gen: cannot read {p}: {e}");
                return ExitCode::from(2);
            }
        },
        None => String::new(),
    };
    let gen = match mode {
        "artifacts" => gen_artifacts(&phext_text, &src_text, &phext_sha, &hop),
        "tasks" => gen_tasks(&phext_text, &src_text, &phext_sha, &hop),
        "skills" => gen_skills(&phext_text, &phext_sha, &hop),
        "state" => gen_state(&phext_text, &phext_sha, &hop),
        _ => unreachable!(),
    };
    let gen = match gen {
        Ok(g) => g,
        Err(e) => {
            eprintln!("kgoat-register-gen ({mode}): {e}; not writing");
            return ExitCode::from(1);
        }
    };
    if !fidelity_ok(&phext_text, &gen.text) {
        eprintln!("kgoat-register-gen ({mode}): FIDELITY FAILURE — an original scroll changed; not writing");
        return ExitCode::from(1);
    }
    if let Err(e) = std::fs::write(&out_path, &gen.text) {
        eprintln!("kgoat-register-gen: cannot write {out_path}: {e}");
        return ExitCode::from(2);
    }
    println!(
        "kgoat-register-gen ({mode}): {} generated nodes · {} · gates PASS · fidelity PASS → {}",
        gen.nodes, gen.report, out_path
    );
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn implode_map(m: HashMap<phext::Coordinate, String>) -> String {
        phext::implode(m)
    }

    #[test]
    fn artifacts_gate_and_fold() {
        let mut scrolls: HashMap<phext::Coordinate, String> = HashMap::new();
        scrolls.insert(phext::to_coordinate(PHEXT_HOME), "# artifacts\n".into());
        let body = "Title: Fern — Bio\nSource: ~/x.md\nInscribed: 2026-11-07\nSHA-256: abc123\n\ntext\n";
        // Use a real-format (64-hex) sha in both header and source.
        let sha = "9f3a085965833969420fcbaf8da6a03adbf2e79acc6372cdcb7ed2918a8f23c9";
        let body = body.replace("abc123", sha);
        scrolls.insert(phext::to_coordinate("20.8.11/9.7.13/8.7.11"), body);
        let text = implode_map(scrolls);
        let src = format!(
            r#"{{"artifacts":[{{"coordinate":"20.8.11/9.7.13/8.7.11","inscribed":"2026-11-07","sha256":"{sha}","source":"~/x.md","title":"Fern — Bio"}}]}}"#
        );
        let g = gen_artifacts(&text, &src, "deadbeef", "2026-11-23").unwrap();
        assert_eq!(g.nodes, 2);
        let graph = kgoat::builder::build(&[("artifacts".to_string(), g.text)]);
        assert!(graph.violations.is_empty(), "{:?}", graph.violations);
        assert_eq!(graph.nodes.len(), 2);
        // A corrupted receipt is refused.
        let bad = src.replace(sha, "0000085965833969420fcbaf8da6a03adbf2e79acc6372cdcb7ed2918a8f23c9");
        assert!(gen_artifacts(&text, &bad, "deadbeef", "2026-11-23").is_err());
    }

    #[test]
    fn tasks_gate_and_fold() {
        let mut scrolls: HashMap<phext::Coordinate, String> = HashMap::new();
        scrolls.insert(phext::to_coordinate(PHEXT_HOME), "# tasks\n".into());
        scrolls.insert(
            phext::to_coordinate("3.3.3/4.3.8/8.2.2"),
            "# Study S009\n".into(),
        );
        let text = implode_map(scrolls);
        let src = r#"{"tasks":[{"coordinate":"3.3.3/4.3.8/8.2.2","issued":"2026-11-13","title":"Study S009","status":"open"}]}"#;
        let g = gen_tasks(&text, src, "deadbeef", "2026-11-23").unwrap();
        assert_eq!(g.nodes, 2);
        let graph = kgoat::builder::build(&[("tasks".to_string(), g.text)]);
        assert!(graph.violations.is_empty(), "{:?}", graph.violations);
        let missing = r#"{"tasks":[{"coordinate":"9.9.9/9.9.9/9.9.9","issued":"2026-11-13","title":"X","status":"open"}]}"#;
        assert!(gen_tasks(&text, missing, "deadbeef", "2026-11-23").is_err());
    }

    #[test]
    fn skills_gate_grouping_and_fold() {
        let skillmd = "---\nname: demo_skill\ndescription: A demo capability.\n---\n\n# Demo\n"
            .to_string();
        let yaml = "name: demo_skill\n".to_string();
        let sha_md = sha256_hex(skillmd.as_bytes());
        let sha_yaml = sha256_hex(yaml.as_bytes());
        let home = format!(
            "Home — skills.phext\n\nIndex (coordinate  skill  file  size  SHA-256 of the source file):\n\
             1.1.1/1.1.1/1.1.1  (this scroll)  Home — the map\n\
             1.2.1/1.1.1/1.1.1  demo_skill  SKILL.md  10 bytes  sha256:{}\n\
             1.2.1/1.1.1/1.1.2  demo_skill  skill.yaml  5 bytes  sha256:{}\n",
            &sha_md[..48],
            &sha_yaml[..48]
        );
        let mut scrolls: HashMap<phext::Coordinate, String> = HashMap::new();
        scrolls.insert(phext::to_coordinate(PHEXT_HOME), home);
        scrolls.insert(phext::to_coordinate("1.2.1/1.1.1/1.1.1"), skillmd);
        scrolls.insert(phext::to_coordinate("1.2.1/1.1.1/1.1.2"), yaml);
        let text = implode_map(scrolls);
        let g = gen_skills(&text, "deadbeef", "2026-11-23").unwrap();
        assert_eq!(g.nodes, 2); // Home + one skill node for two files
        let graph = kgoat::builder::build(&[("skills".to_string(), g.text)]);
        assert!(graph.violations.is_empty(), "{:?}", graph.violations);
        let node = graph
            .nodes
            .values()
            .find(|n| n.coord == "1.1.1/1.3.1/1.2.1")
            .unwrap();
        assert_eq!(node.statement, "A demo capability.");
        // A truncated index (a scroll unindexed) is refused.
        let bad_home = "Home\n\nIndex (coordinate  skill  file  size  SHA-256):\n".to_string();
        let mut scrolls2: HashMap<phext::Coordinate, String> = HashMap::new();
        scrolls2.insert(phext::to_coordinate(PHEXT_HOME), bad_home);
        scrolls2.insert(
            phext::to_coordinate("1.2.1/1.1.1/1.1.1"),
            "---\nname: x\ndescription: y\n---\n".into(),
        );
        assert!(gen_skills(&implode_map(scrolls2), "deadbeef", "2026-11-23").is_err());
    }

    #[test]
    fn state_lines_parse_both_forms() {
        let (ts, s) = parse_state_line("2026-11-06 10:08 CST | 0/17 nodes up; fleet idle");
        assert_eq!(ts, "2026-11-06 10:08 CST");
        assert_eq!(s, "0/17 nodes up; fleet idle");
        let (ts, s) = parse_state_line(
            "32.9.11/10.9.14/12.8.12 | 2026-11-14 04:38 CST | 18/23 nodes up; busy: inference",
        );
        assert_eq!(ts, "2026-11-14 04:38 CST");
        assert_eq!(s, "18/23 nodes up; busy: inference");
    }

    #[test]
    fn state_gate_and_fold() {
        let mut scrolls: HashMap<phext::Coordinate, String> = HashMap::new();
        scrolls.insert(phext::to_coordinate(PHEXT_HOME), "# state\n".into());
        scrolls.insert(
            phext::to_coordinate("24.9.10/11.8.13/9.8.9"),
            "2026-11-06 10:08 CST | 0/17 nodes up; fleet idle\n".into(),
        );
        let text = implode_map(scrolls);
        let g = gen_state(&text, "deadbeef", "2026-11-23").unwrap();
        assert_eq!(g.nodes, 2);
        let graph = kgoat::builder::build(&[("state".to_string(), g.text)]);
        assert!(graph.violations.is_empty(), "{:?}", graph.violations);
        assert_eq!(graph.kind_counts.get("event"), Some(&1));
    }
}
