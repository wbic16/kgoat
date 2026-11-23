//! kgoat-research-gen — KGOAT Stage 2, research leg (SPEC §6 Stage 2,
//! §4.1 seed 2). Reads research.phext and the MOAT registry
//! (registry.json, emitted by tluf's own extractor and sha-asserted
//! against the phext), and writes a working copy — research-graph.phext
//! — carrying the graph layer in volume 1.3.1, beside the tracker
//! material (§2.5). Production research.phext is never modified.
//!
//! The registry's assignments are inherited verbatim (§5.1): KGOAT
//! does not re-judge a single status. Before anything is written the
//! generator verifies that the registry's recorded SHA-256 equals the
//! phext's actual SHA-256 — the assay data and the scrolls can never
//! drift apart silently — and that the registry is total over the
//! phext's scrolls in both directions.
//!
//! Per registry unit, two node scrolls (the ietf pattern):
//!   content node  1.1.1/1.3.1/1.2.<id>   kind by coordinate class
//!   claim node    1.1.1/1.3.1/1.4.<id>   the unit's MOAT assay
//! plus the research work node (graph Home) at 1.1.1/1.3.1/1.1.1.
//! Every generated node carries part_of → the work node.
//!
//! Usage: kgoat-research-gen <research.phext> <registry.json> <out.phext> [hop-stamp]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::process::ExitCode;

use libphext::phext;
use serde::Deserialize;
use sha2::{Digest, Sha256};

// ---------- registry.json (tluf's extractor output) ----------

#[derive(Debug, Deserialize)]
struct Registry {
    phext_sha256: String,
    entries: Vec<RegEntry>,
}

#[derive(Debug, Deserialize)]
struct RegEntry {
    id: u32,
    coordinate: String,
    claim: String,
    status: String,
    basis: String,
    evidence: String,
}

// ---------- helpers ----------

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

/// Kind by the unit's coordinate class — the extractor's own classes
/// (extract_registry.classify): ch.1 §2 concepts, §3 foundations and
/// defined words, §5 areas are concept-space; ch.1 §1 meta and §4
/// registry pages are the tracker's own works, except 1.1.4 — the
/// Research Oracle ledger — which is a metric; paper/study chapters
/// (ch. ≥ 2) are works.
fn kind_of(coord: &str) -> Result<&'static str, String> {
    let groups: Vec<&str> = coord.split('/').collect();
    if groups.len() != 3 {
        return Err(format!("coordinate {coord} does not have three groups"));
    }
    let ch: Vec<&str> = groups[2].split('.').collect();
    if ch.len() != 3 {
        return Err(format!("coordinate {coord} chapter group is not a triple"));
    }
    let chapter: u32 = ch[0].parse().map_err(|_| format!("bad chapter in {coord}"))?;
    let section: u32 = ch[1].parse().map_err(|_| format!("bad section in {coord}"))?;
    let scroll: u32 = ch[2].parse().map_err(|_| format!("bad scroll in {coord}"))?;
    Ok(match (chapter, section, scroll) {
        (1, 2, _) | (1, 3, _) | (1, 5, _) => "concept",
        (1, 1, 4) => "metric",
        (1, 1, _) | (1, 4, _) => "work",
        (1, _, _) => "concept",
        _ => "work",
    })
}

fn basis_name(lower: &str) -> Result<&'static str, String> {
    Ok(match lower {
        "stipulation" => "STIPULATION",
        "verified" => "VERIFIED",
        "refuted" => "REFUTED",
        "attributed" => "ATTRIBUTED",
        "open" => "OPEN",
        "synthesis" => "SYNTHESIS",
        "estimate" => "ESTIMATE",
        "construction" => "CONSTRUCTION",
        other => return Err(format!("unknown basis code `{other}` in registry")),
    })
}

fn status_name(s: &str) -> Result<&'static str, String> {
    Ok(match s {
        "True" => "True",
        "False" => "False",
        "Unknown" => "Unknown",
        "Fiction" => "Fiction",
        other => return Err(format!("unknown status `{other}` in registry")),
    })
}

const HOME: &str = "1.1.1/1.3.1/1.1.1";
const CONTENT_PREFIX: &str = "1.1.1/1.3.1/1.2.";
const CLAIM_PREFIX: &str = "1.1.1/1.3.1/1.4.";

// ---------- generation ----------

struct GenOut {
    text: String,
    nodes: usize,
    sanitized: usize,
    kind_counts: BTreeMap<String, usize>,
    status_counts: BTreeMap<String, usize>,
}

fn generate(phext_text: &str, reg: &Registry, hop: &str) -> Result<GenOut, String> {
    // Totality, both directions: registry entries == the phext's
    // non-empty scrolls, by coordinate.
    let scrolls = phext::explode(phext_text);
    let present: HashSet<String> = scrolls
        .iter()
        .filter(|(_, s)| !s.trim().is_empty())
        .map(|(c, _)| c.to_string())
        .collect();
    let registered: HashSet<String> = reg.entries.iter().map(|e| e.coordinate.clone()).collect();
    if present != registered {
        let only_scrolls = present.difference(&registered).count();
        let only_reg = registered.difference(&present).count();
        return Err(format!(
            "registry is not total over this phext: {only_scrolls} scrolls unregistered, {only_reg} entries without scrolls"
        ));
    }
    let mut ids: HashSet<u32> = HashSet::new();
    for e in &reg.entries {
        if !ids.insert(e.id) {
            return Err(format!("duplicate registry id {}", e.id));
        }
    }

    let mut out: HashMap<phext::Coordinate, String> = scrolls
        .iter()
        .map(|(c, s)| (c.clone(), s.clone()))
        .collect();
    let mut sanitized = 0usize;
    let mut kind_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut status_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut basis_counts: BTreeMap<String, usize> = BTreeMap::new();

    for e in &reg.entries {
        let kind = kind_of(&e.coordinate)?;
        *kind_counts.entry(kind.to_string()).or_default() += 1;
        let status = status_name(&e.status)?;
        *status_counts.entry(status.to_string()).or_default() += 1;
        let basis = basis_name(&e.basis)?;
        *basis_counts.entry(basis.to_string()).or_default() += 1;

        let (claim, c1) = one_line(&e.claim);
        let (evidence_raw, c2) = one_line(&e.evidence);
        sanitized += c1 as usize + c2 as usize;
        // The assay line's fields are " · "-separated: a separator
        // inside the evidence text would break the line's grammar.
        // Replace it with an em dash and count the transformation.
        let evidence_raw = if evidence_raw.contains(" · ") {
            sanitized += 1;
            evidence_raw.replace(" · ", " — ")
        } else {
            evidence_raw
        };
        // A stipulation's ground is its own scroll: where the registry
        // records no pointer ("—"), the assay evidence is the unit's
        // own coordinate. Named in the graph Home.
        let evidence = if evidence_raw.is_empty() || evidence_raw == "-" || evidence_raw == "—" {
            e.coordinate.clone()
        } else {
            evidence_raw
        };

        let content_coord = format!("{CONTENT_PREFIX}{}", e.id);
        let claim_coord = format!("{CLAIM_PREFIX}{}", e.id);

        let mut content = String::new();
        content.push_str(&format!("node · {kind} · {claim}\n"));
        content.push_str(&format!("source · research@{}\n", e.coordinate));
        content.push_str(&format!("registry-id · {}\n", e.id));
        content.push_str(&format!(
            "edge · part_of · {HOME} · w0 · {} · {hop}\n",
            e.coordinate
        ));
        out.insert(phext::to_coordinate(&content_coord), content);

        let mut claim_node = String::new();
        claim_node.push_str(&format!("node · claim · {claim}\n"));
        claim_node.push_str(&format!("unit · research@{}\n", e.coordinate));
        claim_node.push_str(&format!("content-node · {content_coord}\n"));
        claim_node.push_str(&format!("registry-id · {}\n", e.id));
        claim_node.push_str(&format!(
            "assay · {status}/{basis} · {evidence} · {hop}\n"
        ));
        claim_node.push_str(&format!(
            "edge · part_of · {HOME} · w0 · {} · {hop}\n",
            e.coordinate
        ));
        out.insert(phext::to_coordinate(&claim_coord), claim_node);
    }

    // The graph Home: the research work node.
    let total_status: usize = status_counts.values().sum();
    let kinds_line = kind_counts
        .iter()
        .map(|(k, v)| format!("{k} {v}"))
        .collect::<Vec<_>>()
        .join(" · ");
    let status_line = status_counts
        .iter()
        .map(|(k, v)| format!("{k} {v}"))
        .collect::<Vec<_>>()
        .join(" · ");
    let basis_line = basis_counts
        .iter()
        .map(|(k, v)| format!("{k} {v}"))
        .collect::<Vec<_>>()
        .join(" · ");
    let mut home = String::new();
    home.push_str(
        "node · work · research.phext — the R&D tracker (MOAT), as a phext · KGOAT graph layer home\n",
    );
    home.push_str(&format!(
        "The graph layer of research.phext (SPEC §6 Stage 2): {} units of the tracker, each present as a content node (1.2.<registry id>) and a claim node (1.4.<registry id>) carrying its MOAT assay, inherited verbatim from the registry — KGOAT does not re-judge them (§5.1).\n",
        reg.entries.len()
    ));
    home.push_str(&format!(
        "source phext sha256: {} · registry entries: {} · statuses: {} · bases: {}\n",
        reg.phext_sha256, total_status, status_line, basis_line
    ));
    home.push_str(&format!("content kinds: {kinds_line}\n"));
    home.push_str(
        "Kind by coordinate class (the registry extractor's own classes): ch.1 §2 concepts, §3 foundations/defined words, §5 areas → concept; ch.1 §1 meta and §4 registry pages → work, except 1.1.4 (the Research Oracle ledger) → metric; paper/study chapters → work.\n",
    );
    home.push_str(
        "Evidence mappings, the only transformations: where the registry records no evidence pointer (\"—\", the stipulations — true by definition), the claim node's assay evidence is the unit's own coordinate, the scroll where the definition stands; and a \" · \" occurring inside an evidence string is written \" — \", because \" · \" is the assay line's field separator.\n",
    );
    home.push_str(&format!(
        "Generated by kgoat-research-gen · hop {hop} · the scrolls are right; this layer is a reading of them (§2.3).\n"
    ));
    out.insert(phext::to_coordinate(HOME), home);

    let nodes = 1 + 2 * reg.entries.len();
    Ok(GenOut {
        text: phext::implode(out),
        nodes,
        sanitized,
        kind_counts,
        status_counts,
    })
}

// ---------- main ----------

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("usage: kgoat-research-gen <research.phext> <registry.json> <out.phext> [hop-stamp]");
        return ExitCode::from(2);
    }
    let hop = args.get(3).map(|s| s.as_str()).unwrap_or("2026-11-23");
    let phext_bytes = match std::fs::read(&args[0]) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("kgoat-research-gen: cannot read {}: {e}", args[0]);
            return ExitCode::from(2);
        }
    };
    let reg_text = match std::fs::read_to_string(&args[1]) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("kgoat-research-gen: cannot read {}: {e}", args[1]);
            return ExitCode::from(2);
        }
    };
    let reg: Registry = match serde_json::from_str(&reg_text) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("kgoat-research-gen: cannot parse {}: {e}", args[1]);
            return ExitCode::from(2);
        }
    };
    // The gate: the registry must be the registry OF THIS PHEXT.
    let actual = sha256_hex(&phext_bytes);
    if actual != reg.phext_sha256 {
        eprintln!(
            "kgoat-research-gen: SHA MISMATCH — registry is for {}, this phext is {actual}; not writing",
            reg.phext_sha256
        );
        return ExitCode::from(1);
    }
    let phext_text = String::from_utf8_lossy(&phext_bytes).into_owned();
    let before: HashMap<String, String> = phext::explode(&phext_text)
        .iter()
        .map(|(c, s)| (c.to_string(), s.clone()))
        .collect();
    let gen = match generate(&phext_text, &reg, hop) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("kgoat-research-gen: {e}; not writing");
            return ExitCode::from(1);
        }
    };
    // Fidelity: every original scroll byte-identical in the copy.
    let after = phext::explode(&gen.text);
    let mut fidelity_ok = after.len() >= before.len();
    if fidelity_ok {
        for (c, s) in &before {
            match after.iter().find(|(ac, _)| &ac.to_string() == c) {
                Some((_, as_)) if as_ == s => {}
                _ => {
                    fidelity_ok = false;
                    break;
                }
            }
        }
    }
    if !fidelity_ok {
        eprintln!("kgoat-research-gen: FIDELITY FAILURE — an original scroll changed; not writing");
        return ExitCode::from(1);
    }
    if let Err(e) = std::fs::write(&args[2], &gen.text) {
        eprintln!("kgoat-research-gen: cannot write {}: {e}", args[2]);
        return ExitCode::from(2);
    }
    let kinds = gen
        .kind_counts
        .iter()
        .map(|(k, v)| format!("{k} {v}"))
        .collect::<Vec<_>>()
        .join(" · ");
    let statuses = gen
        .status_counts
        .iter()
        .map(|(k, v)| format!("{k} {v}"))
        .collect::<Vec<_>>()
        .join(" · ");
    println!(
        "kgoat-research-gen: {} units · {} generated nodes · kinds: {} · statuses: {} · {} fields adjusted (line-collapsed / separator-safe) · sha gate PASS · fidelity PASS → {}",
        reg.entries.len(),
        gen.nodes,
        kinds,
        statuses,
        gen.sanitized,
        args[2]
    );
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (String, Registry) {
        let mut scrolls: HashMap<phext::Coordinate, String> = HashMap::new();
        scrolls.insert(
            phext::to_coordinate("1.1.1/1.1.1/1.1.4"),
            "# Research Oracle — definition, metric, ledger\n".to_string(),
        );
        scrolls.insert(
            phext::to_coordinate("1.1.1/1.1.1/1.2.1"),
            "# Concept: Intelligence (economic definition)\n".to_string(),
        );
        scrolls.insert(
            phext::to_coordinate("1.1.1/1.1.1/1.3.2"),
            "# Foundation: function, variable, input, output\n".to_string(),
        );
        scrolls.insert(
            phext::to_coordinate("1.1.1/1.1.1/2.1.1"),
            "# Paper 001 delta\n".to_string(),
        );
        let text = phext::implode(scrolls);
        let reg = Registry {
            phext_sha256: String::new(),
            entries: vec![
                RegEntry {
                    id: 1,
                    coordinate: "1.1.1/1.1.1/1.1.4".into(),
                    claim: "The Research Oracle ledger".into(),
                    status: "Unknown".into(),
                    basis: "estimate".into(),
                    evidence: "the ledger itself".into(),
                },
                RegEntry {
                    id: 2,
                    coordinate: "1.1.1/1.1.1/1.2.1".into(),
                    claim: "Intelligence (economic definition)".into(),
                    status: "Unknown".into(),
                    basis: "attributed".into(),
                    evidence: "Paper 001, §1".into(),
                },
                RegEntry {
                    id: 3,
                    coordinate: "1.1.1/1.1.1/1.3.2".into(),
                    claim: "Foundation: function, variable, input, output".into(),
                    status: "True".into(),
                    basis: "stipulation".into(),
                    evidence: "—".into(),
                },
                RegEntry {
                    id: 4,
                    coordinate: "1.1.1/1.1.1/2.1.1".into(),
                    claim: "Paper 001 delta".into(),
                    status: "Unknown".into(),
                    basis: "synthesis".into(),
                    evidence: "the scroll itself".into(),
                },
            ],
        };
        (text, reg)
    }

    #[test]
    fn kinds_follow_coordinate_classes() {
        assert_eq!(kind_of("1.1.1/1.1.1/1.2.485").unwrap(), "concept");
        assert_eq!(kind_of("1.1.1/1.1.1/1.3.2").unwrap(), "concept");
        assert_eq!(kind_of("1.1.1/1.1.1/1.5.4").unwrap(), "concept");
        assert_eq!(kind_of("1.1.1/1.1.1/1.1.4").unwrap(), "metric");
        assert_eq!(kind_of("1.1.1/1.1.1/1.1.1").unwrap(), "work");
        assert_eq!(kind_of("1.1.1/1.1.1/1.4.2").unwrap(), "work");
        assert_eq!(kind_of("1.1.1/1.1.1/53.1.6").unwrap(), "work");
    }

    #[test]
    fn generation_is_total_and_maps_stipulation_evidence() {
        let (text, reg) = fixture();
        let gen = generate(&text, &reg, "2026-11-23").unwrap();
        assert_eq!(gen.nodes, 9);
        let scrolls = phext::explode(&gen.text);
        let claim3 = scrolls
            .iter()
            .find(|(c, _)| c.to_string() == "1.1.1/1.3.1/1.4.3")
            .map(|(_, s)| s.clone())
            .unwrap();
        // The stipulation's "—" became its own coordinate.
        assert!(
            claim3.contains("assay · True/STIPULATION · 1.1.1/1.1.1/1.3.2 · 2026-11-23"),
            "{claim3}"
        );
        // And the whole layer folds clean through the builder.
        let g = kgoat::builder::build(&[("research".to_string(), gen.text)]);
        assert!(g.violations.is_empty(), "{:?}", g.violations);
        assert_eq!(g.nodes.len(), 9);
        assert_eq!(g.registry.len(), 4);
        assert_eq!(g.status_counts.get("True"), Some(&1));
        assert_eq!(g.status_counts.get("Unknown"), Some(&3));
        assert_eq!(g.edges_to_non_nodes, 0);
    }

    #[test]
    fn non_total_registry_is_refused() {
        let (text, mut reg) = fixture();
        reg.entries.pop();
        assert!(generate(&text, &reg, "2026-11-23").is_err());
    }
}
