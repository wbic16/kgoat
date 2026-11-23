//! kgoat-ietf-gen — the KGOAT Stage 1 generator (SPEC §6): reads
//! ietf.phext's stored edges into node scrolls in a working copy of the
//! phext itself.
//!
//! The graph layer lives in book 1.3.1 of volume 1.1.1, beside the
//! material (SPEC §2.5): the corpus as a `work` node at 1.1.1/1.3.1/1.1.1,
//! one `document` node per RFC at 1.1.1/1.3.1/1.2.<rfc>, one `project`
//! node per implementing project at 1.1.1/1.3.1/1.3.<n>, and one `claim`
//! node per MOAT record at 1.1.1/1.3.1/1.4.<rfc>.
//!
//! Edges are drawn only in the frozen v1 types (SPEC §1.3): references
//! (normative, informative, and kind-unspecified, grouped by section
//! heads in the scroll), supersedes (from `obsoletes` only — the
//! reverse is derived, never stored), implements (stored on the
//! project node), part_of (to the corpus work node). The corpus's
//! `updates` relations have no v1 edge type; they are recorded as
//! provenance text on the document node and named as a gap in the
//! graph Home — never drawn under a wrong type.
//!
//! Weights: the source stores none, so every extracted edge is w0 —
//! recorded, no strength judged (SPEC §1.3: copied where stored, never
//! defaulted to 3).
//!
//! Data sources. ietf.phext's Home scroll declares ietf-source.json
//! the corpus's source of truth, and the document scrolls truncate
//! long reference lists at 60 entries with a "(+N more in
//! ietf-source.json)" marker — so the complete edge lists are read
//! from the JSON, and the scrolls are the cross-check: every list the
//! scrolls print must be an exact prefix of the JSON list (with the
//! truncation marker accounting for the remainder, exactly). Any other
//! disagreement is a divergence: reported, and generation stops.
//!
//! The production phext is never touched: this writes a working copy
//! and proves, before writing, that every original scroll survives
//! the explode→implode round trip byte-identical.
//!
//! Usage: `kgoat-ietf-gen <ietf.phext> <ietf-source.json> <out.phext> [hop-stamp]`

use std::collections::{BTreeMap, HashMap};
use std::process::ExitCode;

use libphext::phext;
use serde::Deserialize;

const WORK_COORD: &str = "1.1.1/1.3.1/1.1.1";
const HOME_COORD: &str = "1.1.1/1.1.1/1.1.1";

fn doc_node_coord(rfc: u32) -> String {
    format!("1.1.1/1.3.1/1.2.{rfc}")
}
fn claim_node_coord(rfc: u32) -> String {
    format!("1.1.1/1.3.1/1.4.{rfc}")
}
fn project_node_coord(n: usize) -> String {
    format!("1.1.1/1.3.1/1.3.{n}")
}
fn doc_scroll_coord(rfc: u32) -> String {
    format!("1.1.1/1.2.1/1.1.{rfc}")
}

// ---------- ietf-source.json ----------
//
// The source records nulls for absent values (a placeholder's title,
// an unmapped entry's text hash, …). Raw structs therefore take
// Option everywhere a value may be absent; `plain` normalizes once,
// up front, into the plain structs the generator works with.

#[derive(Debug, Deserialize)]
struct RawSource {
    entries: Vec<RawEntry>,
}

#[derive(Debug, Deserialize)]
struct RawEntry {
    rfc: u32,
    title: Option<String>,
    authors: Option<Vec<String>>,
    current_status: Option<String>,
    record_status: Option<String>,
    record_basis: Option<String>,
    text_sha256: Option<String>,
    implementations: Option<Vec<RawImpl>>,
    edges: RawEdges,
}

#[derive(Debug, Deserialize)]
struct RawImpl {
    name: Option<String>,
    repo_url: Option<String>,
    license: Option<String>,
    language: Option<String>,
    role: Option<String>,
    basis: Option<String>,
    evidence: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawEdges {
    index: RawIndexEdges,
    references: Option<Vec<RawRefEdge>>,
}

#[derive(Debug, Deserialize)]
struct RawIndexEdges {
    obsoletes: Option<Vec<u32>>,
    updates: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize)]
struct RawRefEdge {
    to_rfc: u32,
    kind: Option<String>,
}

#[derive(Debug)]
struct Source {
    entries: Vec<Entry>,
}

#[derive(Debug)]
struct Entry {
    rfc: u32,
    title: String,
    authors: Vec<String>,
    current_status: String,
    record_status: String,
    record_basis: String,
    text_sha256: String,
    implementations: Vec<Impl>,
    edges: Edges,
}

#[derive(Debug)]
struct Impl {
    name: String,
    repo_url: String,
    license: String,
    language: String,
    role: String,
    basis: String,
    evidence: String,
}

#[derive(Debug)]
struct Edges {
    index: IndexEdges,
    references: Vec<RefEdge>,
}

#[derive(Debug)]
struct IndexEdges {
    obsoletes: Vec<u32>,
    updates: Vec<u32>,
}

#[derive(Debug)]
struct RefEdge {
    to_rfc: u32,
    kind: String,
}

fn plain(raw: RawSource) -> Source {
    let os = |o: Option<String>| o.unwrap_or_default();
    Source {
        entries: raw
            .entries
            .into_iter()
            .map(|e| Entry {
                rfc: e.rfc,
                title: os(e.title),
                authors: e.authors.unwrap_or_default(),
                current_status: os(e.current_status),
                record_status: os(e.record_status),
                record_basis: os(e.record_basis),
                text_sha256: os(e.text_sha256),
                implementations: e
                    .implementations
                    .unwrap_or_default()
                    .into_iter()
                    .map(|i| Impl {
                        name: os(i.name),
                        repo_url: os(i.repo_url),
                        license: os(i.license),
                        language: os(i.language),
                        role: os(i.role),
                        basis: os(i.basis),
                        evidence: os(i.evidence),
                    })
                    .collect(),
                edges: Edges {
                    index: IndexEdges {
                        obsoletes: e.edges.index.obsoletes.unwrap_or_default(),
                        updates: e.edges.index.updates.unwrap_or_default(),
                    },
                    references: e
                        .edges
                        .references
                        .unwrap_or_default()
                        .into_iter()
                        .map(|r| RefEdge {
                            to_rfc: r.to_rfc,
                            kind: os(r.kind),
                        })
                        .collect(),
                },
            })
            .collect(),
    }
}

// ---------- the scroll-side cross-check view ----------

/// What one document scroll actually prints in its header — used only
/// to verify the JSON reading, never as the edge source.
#[derive(Debug, Default)]
struct ScrollView {
    refs_normative: Vec<u32>,
    refs_informative: Vec<u32>,
    refs_unspecified: Vec<u32>,
    /// Per-kind truncation remainder claimed by a "(+N more …)" marker.
    trunc_normative: Option<usize>,
    trunc_informative: Option<usize>,
    trunc_unspecified: Option<usize>,
    obsoletes: Vec<u32>,
    updates: Vec<u32>,
    impl_names: Vec<String>,
}

/// Remove annotation groups — "(N edges)" counts and "(+N more in
/// ietf-source.json)" truncation markers — so their numbers are not
/// mistaken for RFC references.
fn strip_annotations(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(open) = rest.find('(') {
        out.push_str(&rest[..open]);
        match rest[open..].find(')') {
            Some(close) => {
                let inner = &rest[open + 1..open + close];
                if !(inner.contains("edges") || inner.contains("more in")) {
                    out.push_str(&rest[open..=open + close]);
                }
                rest = &rest[open + close + 1..];
            }
            None => {
                out.push_str(&rest[open..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

/// The remainder N of a "(+N more in ietf-source.json)" marker, if present.
fn truncation_remainder(s: &str) -> Option<usize> {
    let start = s.find("(+")?;
    let tail = &s[start + 2..];
    let end = tail.find(' ')?;
    tail[..end].parse::<usize>().ok()
}

/// Every integer in `s`, in order, once annotations are stripped.
fn extract_numbers(s: &str) -> Vec<u32> {
    let cleaned = strip_annotations(s);
    let mut nums = Vec::new();
    let mut cur = String::new();
    for ch in cleaned.chars() {
        if ch.is_ascii_digit() {
            cur.push(ch);
        } else {
            if let Ok(n) = cur.parse::<u32>() {
                nums.push(n);
            }
            cur.clear();
        }
    }
    if let Ok(n) = cur.parse::<u32>() {
        nums.push(n);
    }
    nums
}

/// Parse the bracket groups of an "Index edges:" segment:
/// `obsoletes [..]; obsoleted by [..]; updates [..]; updated by [..]`.
fn parse_index_edges(seg: &str) -> (Vec<u32>, Vec<u32>) {
    let mut obsoletes = Vec::new();
    let mut updates = Vec::new();
    let seg = seg.strip_prefix("Index edges:").unwrap_or(seg);
    for part in seg.split(';') {
        let part = part.trim();
        let grab = |p: &str| -> Vec<u32> {
            match (p.find('['), p.rfind(']')) {
                (Some(a), Some(b)) if b > a => extract_numbers(&p[a + 1..b]),
                _ => Vec::new(),
            }
        };
        if part.starts_with("obsoletes") {
            obsoletes = grab(part);
        } else if part.starts_with("updates") {
            updates = grab(part);
        }
    }
    (obsoletes, updates)
}

/// Parse one implementation entry line:
/// `- <name> (<role>) -- <url> -- license <lic> -- <lang> -- basis <BASIS>`
/// Only the name is needed for the cross-check. The name itself may
/// carry a parenthetical qualifier ("CPython (stdlib)"), so the role
/// is the LAST parenthesized group before the first " -- ".
fn parse_impl_name(line: &str) -> Option<String> {
    let body = line.trim().strip_prefix("- ")?;
    let head = body.split(" -- ").next()?;
    let name = match head.rfind(" (") {
        Some(p) if head.ends_with(')') => head[..p].trim().to_string(),
        _ => head.trim().to_string(),
    };
    Some(name)
}

/// Read the verification view out of one RFC scroll's header. The
/// header ends at the scroll's own "Coordinate:" line or at the RFC
/// text marker, whichever comes first.
fn parse_scroll_view(text: &str) -> ScrollView {
    #[derive(PartialEq, Clone, Copy)]
    enum Seg {
        None,
        Index,
        Norm,
        Info,
        Unspec,
        Other,
    }
    let mut view = ScrollView::default();
    let mut seg = Seg::None;
    let mut buf = String::new();
    let mut in_impl = false;
    let flush = |seg: Seg, buf: &str, view: &mut ScrollView| match seg {
        Seg::Index => {
            let (o, u) = parse_index_edges(buf);
            view.obsoletes = o;
            view.updates = u;
        }
        Seg::Norm => {
            view.refs_normative = extract_numbers(buf);
            view.trunc_normative = truncation_remainder(buf);
        }
        Seg::Info => {
            view.refs_informative = extract_numbers(buf);
            view.trunc_informative = truncation_remainder(buf);
        }
        Seg::Unspec => {
            view.refs_unspecified = extract_numbers(buf);
            view.trunc_unspecified = truncation_remainder(buf);
        }
        _ => {}
    };
    for raw in text.lines() {
        let t = raw.trim();
        if t.starts_with("--- RFC TEXT AS SERVED") {
            break;
        }
        if t.starts_with("Coordinate: ") {
            // The header's last line; never segment content.
            break;
        }
        if t.starts_with("Implementations") {
            flush(seg, &buf, &mut view);
            seg = Seg::None;
            buf.clear();
            in_impl = true;
            continue;
        }
        if t.starts_with("Source: ") {
            flush(seg, &buf, &mut view);
            seg = Seg::None;
            buf.clear();
            in_impl = false;
            continue;
        }
        if in_impl {
            if t.starts_with("- ") {
                if let Some(name) = parse_impl_name(t) {
                    view.impl_names.push(name);
                }
            }
            continue;
        }
        if t.starts_with("Index edges:") {
            flush(seg, &buf, &mut view);
            seg = Seg::Index;
            buf = t.to_string();
            continue;
        }
        if t.starts_with("Normative references") {
            flush(seg, &buf, &mut view);
            seg = Seg::Norm;
            buf = t.to_string();
            continue;
        }
        if t.starts_with("Informative references") {
            flush(seg, &buf, &mut view);
            seg = Seg::Info;
            buf = t.to_string();
            continue;
        }
        if t.starts_with("References (kind unspecified") {
            flush(seg, &buf, &mut view);
            seg = Seg::Unspec;
            buf = t.to_string();
            continue;
        }
        if t.starts_with("Cited by") {
            flush(seg, &buf, &mut view);
            seg = Seg::Other;
            buf.clear();
            continue;
        }
        if matches!(seg, Seg::Index | Seg::Norm | Seg::Info | Seg::Unspec) && !t.is_empty() {
            buf.push(' ');
            buf.push_str(t);
        }
    }
    flush(seg, &buf, &mut view);
    view
}

/// One list, verified: the scroll's printed numbers must be an exact
/// prefix of the JSON list, and any shortfall must be accounted for,
/// exactly, by the scroll's own truncation marker.
fn verify_list(
    rfc: u32,
    what: &str,
    printed: &[u32],
    trunc: Option<usize>,
    full: &[u32],
    divergences: &mut Vec<String>,
) {
    if printed.len() > full.len() {
        divergences.push(format!(
            "RFC {rfc} {what}: scroll prints {} entries, JSON holds {} — scroll is not a prefix",
            printed.len(),
            full.len()
        ));
        return;
    }
    if printed != &full[..printed.len()] {
        divergences.push(format!(
            "RFC {rfc} {what}: scroll's printed entries are not a prefix of the JSON list"
        ));
        return;
    }
    let shortfall = full.len() - printed.len();
    match (shortfall, trunc) {
        (0, _) => {}
        (n, Some(m)) if n == m => {}
        (n, Some(m)) => divergences.push(format!(
            "RFC {rfc} {what}: truncation marker claims +{m} more, actual shortfall is {n}"
        )),
        (n, None) => divergences.push(format!(
            "RFC {rfc} {what}: scroll prints {} of {} entries (short {n}) with no truncation marker",
            printed.len(),
            full.len()
        )),
    }
}

fn refs_of_kind(entry: &Entry, kind: &str) -> Vec<u32> {
    entry
        .edges
        .references
        .iter()
        .filter(|r| r.kind == kind)
        .map(|r| r.to_rfc)
        .collect()
}

fn edge_line(ty: &str, target: &str, evidence: &str, hop: &str) -> String {
    format!("edge · {ty} · {target} · w0 · {evidence} · {hop}")
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!("usage: kgoat-ietf-gen <ietf.phext> <ietf-source.json> <out.phext> [hop-stamp]");
        return ExitCode::from(2);
    }
    let input = match std::fs::read_to_string(&args[0]) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("kgoat-ietf-gen: cannot read {}: {e}", args[0]);
            return ExitCode::from(2);
        }
    };
    let source_text = match std::fs::read_to_string(&args[1]) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("kgoat-ietf-gen: cannot read {}: {e}", args[1]);
            return ExitCode::from(2);
        }
    };
    let source: Source = match serde_json::from_str::<RawSource>(&source_text) {
        Ok(s) => plain(s),
        Err(e) => {
            eprintln!("kgoat-ietf-gen: cannot parse {}: {e}", args[1]);
            return ExitCode::from(2);
        }
    };
    let out_path = &args[2];
    let hop = args.get(3).cloned().unwrap_or_else(|| "2026-11-23".to_string());
    let input_checksum = phext::checksum(&input);

    let scrolls = phext::explode(&input);
    let mut original: Vec<(String, String)> = scrolls
        .iter()
        .map(|(c, s)| (c.to_string(), s.clone()))
        .collect();
    original.sort();
    let scroll_by_coord: HashMap<&str, &str> = original
        .iter()
        .map(|(c, s)| (c.as_str(), s.as_str()))
        .collect();
    eprintln!(
        "kgoat-ietf-gen: {} scrolls · {} JSON entries · input libphext checksum {}",
        original.len(),
        source.entries.len(),
        input_checksum
    );

    // Verify the JSON reading against what the scrolls print.
    let mut divergences: Vec<String> = Vec::new();
    let mut views: HashMap<u32, ScrollView> = HashMap::new();
    for entry in &source.entries {
        let coord = doc_scroll_coord(entry.rfc);
        let text = match scroll_by_coord.get(coord.as_str()) {
            Some(t) => *t,
            None => {
                divergences.push(format!("RFC {}: no document scroll at {coord}", entry.rfc));
                continue;
            }
        };
        let view = parse_scroll_view(text);
        verify_list(
            entry.rfc,
            "references/normative",
            &view.refs_normative,
            view.trunc_normative,
            &refs_of_kind(entry, "normative"),
            &mut divergences,
        );
        verify_list(
            entry.rfc,
            "references/informative",
            &view.refs_informative,
            view.trunc_informative,
            &refs_of_kind(entry, "informative"),
            &mut divergences,
        );
        verify_list(
            entry.rfc,
            "references/unspecified",
            &view.refs_unspecified,
            view.trunc_unspecified,
            &refs_of_kind(entry, "unspecified"),
            &mut divergences,
        );
        verify_list(
            entry.rfc,
            "obsoletes",
            &view.obsoletes,
            None,
            &entry.edges.index.obsoletes,
            &mut divergences,
        );
        verify_list(
            entry.rfc,
            "updates",
            &view.updates,
            None,
            &entry.edges.index.updates,
            &mut divergences,
        );
        let json_impl_names: Vec<String> =
            entry.implementations.iter().map(|i| i.name.clone()).collect();
        if view.impl_names != json_impl_names {
            // The scroll prints "Implementations: UNMAPPED" prose when no
            // mapping was attempted; that is a stored absence, and the
            // JSON must agree it is empty.
            if !(view.impl_names.is_empty() && json_impl_names.is_empty()) {
                divergences.push(format!(
                    "RFC {} implementations: scroll prints {:?}, JSON holds {:?}",
                    entry.rfc, view.impl_names, json_impl_names
                ));
            }
        }
        views.insert(entry.rfc, view);
    }
    if !divergences.is_empty() {
        eprintln!(
            "kgoat-ietf-gen: {} DIVERGENCES between the scrolls and ietf-source.json:",
            divergences.len()
        );
        for d in divergences.iter().take(50) {
            eprintln!("  divergence: {d}");
        }
        eprintln!("kgoat-ietf-gen: NOT writing {out_path} — the readings disagree");
        return ExitCode::from(1);
    }
    eprintln!("kgoat-ietf-gen: scroll/JSON cross-check PASS (all lists verified as prefixes with exact truncation accounting)");

    // Invert implementations into projects (alphabetical = node order).
    #[derive(Default)]
    struct Project {
        url: String,
        license: String,
        language: String,
        links: Vec<(u32, String, String, String)>, // (rfc, role, basis, evidence)
    }
    let mut projects: BTreeMap<String, Project> = BTreeMap::new();
    let mut impl_total = 0usize;
    for entry in &source.entries {
        for link in &entry.implementations {
            impl_total += 1;
            let p = projects.entry(link.name.clone()).or_default();
            if p.url.is_empty() {
                p.url = link.repo_url.clone();
                p.license = link.license.clone();
                p.language = link.language.clone();
            }
            p.links.push((
                entry.rfc,
                link.role.clone(),
                link.basis.clone(),
                link.evidence.clone(),
            ));
        }
    }

    let refs_total: usize = source.entries.iter().map(|e| e.edges.references.len()).sum();
    let refs_normative: usize = source
        .entries
        .iter()
        .map(|e| refs_of_kind(e, "normative").len())
        .sum();
    let refs_informative: usize = source
        .entries
        .iter()
        .map(|e| refs_of_kind(e, "informative").len())
        .sum();
    let refs_unspecified: usize = source
        .entries
        .iter()
        .map(|e| refs_of_kind(e, "unspecified").len())
        .sum();
    let obsoletes_total: usize = source
        .entries
        .iter()
        .map(|e| e.edges.index.obsoletes.len())
        .sum();
    let updates_total: usize = source
        .entries
        .iter()
        .map(|e| e.edges.index.updates.len())
        .sum();
    let record_pairs: usize = source
        .entries
        .iter()
        .filter(|e| !e.record_status.is_empty() && !e.record_basis.is_empty())
        .count();
    let truncated_scrolls: usize = views
        .values()
        .filter(|v| {
            v.trunc_normative.is_some() || v.trunc_informative.is_some() || v.trunc_unspecified.is_some()
        })
        .count();

    // Build the union map: original scrolls + generated node scrolls.
    let mut map: HashMap<phext::Coordinate, String> = HashMap::new();
    for (coord, text) in &original {
        map.insert(phext::to_coordinate(coord), text.clone());
    }
    let mut generated: Vec<String> = Vec::new();
    let put = |map: &mut HashMap<phext::Coordinate, String>,
               generated: &mut Vec<String>,
               coord: &str,
               text: String| {
        map.insert(phext::to_coordinate(coord), text);
        generated.push(coord.to_string());
    };

    let node_count = source.entries.len() + projects.len() + record_pairs + 1;
    let part_of_total = source.entries.len() + projects.len() + record_pairs;
    let mut home = String::new();
    home.push_str("node · work · ietf.phext — the Internet standards, as a phext · KGOAT graph layer home\n");
    home.push_str(&format!(
        "provenance · graph layer generated by kgoat-ietf-gen {} · {hop} · source ietf.phext libphext checksum {}\n\n",
        env!("CARGO_PKG_VERSION"),
        input_checksum
    ));
    home.push_str("The KGOAT graph layer over ietf.phext (SPEC §6, Stage 1). The node scrolls in book 1.3.1 are the storage; anything derived from them is generated, never maintained (SPEC §2.3).\n\n");
    home.push_str(&format!(
        "nodes: {} · document {} · project {} · claim {} · work 1\n",
        node_count,
        source.entries.len(),
        projects.len(),
        record_pairs
    ));
    home.push_str(&format!(
        "edges: references {} (normative {} · informative {} · kind-unspecified {}) · supersedes {} · implements {} · part_of {}\n",
        refs_total, refs_normative, refs_informative, refs_unspecified, obsoletes_total, impl_total, part_of_total
    ));
    home.push_str("weights: every extracted edge is w0 — the source stores no weights; weighting is a judgment pass, not an extraction output (SPEC §1.3).\n");
    home.push_str(&format!(
        "updates: {} update relations are recorded on the document nodes as provenance text and are NOT drawn as edges — the frozen v1 catalogue has no updates type (named gap).\n",
        updates_total
    ));
    home.push_str(&format!(
        "sources: edge lists are read complete from ietf-source.json, the corpus's declared source of truth (ietf.phext Home); the document scrolls truncate long reference lists at 60 entries with a (+N more in ietf-source.json) marker — {} scrolls carry such a marker — and every printed list was verified as an exact prefix of the JSON list before generation.\n",
        truncated_scrolls
    ));
    home.push_str(&format!(
        "record claims: {} of {} documents carry a stored MOAT record pair; each is a claim node at 1.1.1/1.3.1/1.4.<rfc>.\n",
        record_pairs,
        source.entries.len()
    ));
    put(&mut map, &mut generated, WORK_COORD, home);

    for entry in &source.entries {
        let dscroll = doc_scroll_coord(entry.rfc);
        let status = if entry.current_status.is_empty() {
            "annotated record".to_string()
        } else {
            entry.current_status.clone()
        };
        let mut s = String::new();
        s.push_str(&format!(
            "node · document · RFC {} — {} · {}\n",
            entry.rfc, entry.title, status
        ));
        if !entry.authors.is_empty() {
            s.push_str(&format!(
                "provenance · {} — authors as recorded in ietf.phext at {}\n",
                entry.authors.join(", "),
                dscroll
            ));
        }
        if !entry.record_status.is_empty() && !entry.record_basis.is_empty() {
            s.push_str(&format!(
                "record · {}/{} — the document's MOAT record, carried as the claim node at {}\n",
                entry.record_status,
                entry.record_basis,
                claim_node_coord(entry.rfc)
            ));
        }
        if !entry.edges.index.updates.is_empty() {
            let list: Vec<String> = entry
                .edges
                .index
                .updates
                .iter()
                .map(|n| format!("RFC {n}"))
                .collect();
            s.push_str(&format!(
                "updates-record · this document updates {} — recorded here as provenance; the v1 edge catalogue has no updates type, so no edge is drawn (named gap)\n",
                list.join(", ")
            ));
        }
        for (kind, head) in [
            ("normative", "— references (normative) —"),
            ("informative", "— references (informative) —"),
            ("unspecified", "— references (kind unspecified in source) —"),
        ] {
            let refs = refs_of_kind(entry, kind);
            if !refs.is_empty() {
                s.push_str(head);
                s.push('\n');
                for to in &refs {
                    s.push_str(&edge_line("references", &doc_node_coord(*to), &dscroll, &hop));
                    s.push('\n');
                }
            }
        }
        if !entry.edges.index.obsoletes.is_empty() {
            s.push_str("— supersedes —\n");
            for to in &entry.edges.index.obsoletes {
                s.push_str(&edge_line("supersedes", &doc_node_coord(*to), &dscroll, &hop));
                s.push('\n');
            }
        }
        s.push_str(&edge_line("part_of", WORK_COORD, &dscroll, &hop));
        s.push('\n');
        put(&mut map, &mut generated, &doc_node_coord(entry.rfc), s);

        if !entry.record_status.is_empty() && !entry.record_basis.is_empty() {
            let mut c = String::new();
            c.push_str(&format!(
                "node · claim · RFC {} record — existence and text identity of the publication as recorded by the RFC Editor index\n",
                entry.rfc
            ));
            c.push_str(&format!(
                "assay · {}/{} · {dscroll} · {hop}\n",
                entry.record_status, entry.record_basis
            ));
            if !entry.text_sha256.is_empty() {
                c.push_str(&format!(
                    "record · text sha256 (as fetched): {}\n",
                    entry.text_sha256
                ));
            }
            c.push_str(&edge_line("part_of", WORK_COORD, &dscroll, &hop));
            c.push('\n');
            put(&mut map, &mut generated, &claim_node_coord(entry.rfc), c);
        }
    }

    for (i, (name, p)) in projects.iter().enumerate() {
        let coord = project_node_coord(i + 1);
        let mut s = String::new();
        s.push_str(&format!(
            "node · project · {} — {} implementation project · {}\n",
            name, p.language, p.url
        ));
        s.push_str(&format!(
            "provenance · license {} · language {} · as recorded in ietf.phext implementation sections\n",
            p.license, p.language
        ));
        for (rfc, role, basis, evidence) in &p.links {
            s.push_str(&format!(
                "implements-record · RFC {rfc} · {role} · {basis} · {evidence}\n"
            ));
        }
        for (rfc, _, _, _) in &p.links {
            s.push_str(&edge_line(
                "implements",
                &doc_node_coord(*rfc),
                &doc_scroll_coord(*rfc),
                &hop,
            ));
            s.push('\n');
        }
        let first_evidence = p
            .links
            .first()
            .map(|(rfc, _, _, _)| doc_scroll_coord(*rfc))
            .unwrap_or_else(|| HOME_COORD.to_string());
        s.push_str(&edge_line("part_of", WORK_COORD, &first_evidence, &hop));
        s.push('\n');
        put(&mut map, &mut generated, &coord, s);
    }

    let out_text = phext::implode(map);

    // Fidelity proof: every original scroll survives byte-identical, and
    // the only additions are the generated node scrolls.
    let check = phext::explode(&out_text);
    let mut check_map: HashMap<String, String> = check
        .iter()
        .map(|(c, s)| (c.to_string(), s.clone()))
        .collect();
    let mut fidelity_ok = true;
    for (coord, text) in &original {
        match check_map.remove(coord) {
            Some(t) if t == *text => {}
            _ => {
                eprintln!("FIDELITY FAILURE at {coord}");
                fidelity_ok = false;
            }
        }
    }
    let mut extras: Vec<String> = check_map.keys().cloned().collect();
    extras.sort();
    let mut gen_sorted = generated.clone();
    gen_sorted.sort();
    if extras != gen_sorted {
        eprintln!(
            "FIDELITY FAILURE: output extras ({}) do not equal the generated set ({})",
            extras.len(),
            gen_sorted.len()
        );
        fidelity_ok = false;
    }
    if !fidelity_ok {
        eprintln!("kgoat-ietf-gen: NOT writing {out_path} — fidelity proof failed");
        return ExitCode::from(1);
    }
    if let Err(e) = std::fs::write(out_path, &out_text) {
        eprintln!("kgoat-ietf-gen: cannot write {out_path}: {e}");
        return ExitCode::from(2);
    }
    println!(
        "kgoat-ietf-gen: {} documents · {} projects · {} record pairs · references {} · supersedes {} · implements {} · updates (prose) {} · {} generated scrolls · cross-check PASS · fidelity PASS → {}",
        source.entries.len(),
        projects.len(),
        record_pairs,
        refs_total,
        obsoletes_total,
        impl_total,
        updates_total,
        generated.len(),
        out_path
    );
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "# RFC 9110 -- HTTP Semantics\n\
\n\
RFC: 9110\n\
Title: HTTP Semantics\n\
Dependency edges (also in ietf-source.json: edges):\n\
  Index edges: obsoletes [2818, 7230]; obsoleted by [--]; updates [3864]; updated by [--]\n\
  Normative references (from text): 793, 2119 (2 edges)\n\
  Informative references (from text): 1919,\n\
    1945 (2 edges)\n\
  References (kind unspecified in source) (from text): 54 (1 edges)\n\
  Cited by (3 edges, reverse index): normative: 7414; informative: 7605\n\
Implementations (canonical, rule override):\n\
  - curl (canonical) -- https://github.com/curl/curl -- license curl -- C -- basis VERIFIED\n\
    evidence: checked 2026-11-18: git ls-remote resolved\n\
  - CPython (stdlib) (also-implemented) -- https://github.com/python/cpython -- license PSF-2.0 -- Python -- basis VERIFIED\n\
Source: https://www.rfc-editor.org/rfc/rfc9110.txt\n\
Text SHA-256 (as fetched): abc123\n\
Coordinate: 1.1.1/1.2.1/1.1.9110\n\
\n\
--- RFC TEXT AS SERVED BY THE RFC EDITOR ---\n\
body 9999 not a reference\n";

    #[test]
    fn scroll_view_reads_all_reference_kinds() {
        let view = parse_scroll_view(FIXTURE);
        assert_eq!(view.refs_normative, vec![793, 2119]);
        assert_eq!(view.refs_informative, vec![1919, 1945]);
        assert_eq!(view.refs_unspecified, vec![54]);
        assert_eq!(view.obsoletes, vec![2818, 7230]);
        assert_eq!(view.updates, vec![3864]);
        assert_eq!(
            view.impl_names,
            vec!["curl".to_string(), "CPython (stdlib)".to_string()]
        );
    }

    #[test]
    fn cited_by_numbers_are_not_references() {
        // The "(N edges, reverse index)" cited-by form carries RFC
        // numbers that must never enter a references list.
        let view = parse_scroll_view(FIXTURE);
        assert!(!view.refs_normative.contains(&7414));
        assert!(!view.refs_informative.contains(&7605));
    }

    #[test]
    fn verify_list_accepts_exact_prefix_with_marker() {
        let mut divs = Vec::new();
        verify_list(6632, "references/informative", &[768, 793], Some(2), &[768, 793, 951, 1021], &mut divs);
        assert!(divs.is_empty(), "{divs:?}");
    }

    #[test]
    fn verify_list_rejects_unmarked_shortfall() {
        let mut divs = Vec::new();
        verify_list(1, "references/normative", &[793], None, &[793, 2119], &mut divs);
        assert_eq!(divs.len(), 1);
    }

    #[test]
    fn verify_list_rejects_non_prefix() {
        let mut divs = Vec::new();
        verify_list(1, "references/normative", &[999], None, &[793, 2119], &mut divs);
        assert_eq!(divs.len(), 1);
    }

    #[test]
    fn truncation_marker_is_not_a_reference() {
        let nums = extract_numbers("768, 793, ... (+187 more in ietf-source.json) (247 edges)");
        assert_eq!(nums, vec![768, 793]);
        assert_eq!(
            truncation_remainder("768, ... (+187 more in ietf-source.json) (247 edges)"),
            Some(187)
        );
    }
}
