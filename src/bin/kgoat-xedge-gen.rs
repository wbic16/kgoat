//! kgoat-xedge-gen — KGOAT Stage 2, cross-corpus leg (SPEC §6
//! Stage 2; DESIGN.md Addendum 7). Adds the first edges beyond a
//! single register's part_of skeleton: the Research Oracle metric
//! node's `measures` edges, curated from the Oracle ledger.
//!
//!   kgoat-xedge-gen <research.phext> <registry.json> <research-graph.phext> <out.phext> [hop]
//!
//! The (phext, registry) pair is sha-gated exactly as in
//! kgoat-research-gen. The paper set is computed from the registry
//! — chapters ≥ 2 whose §1.1 unit's claim opens with "Paper " —
//! never hand-listed: those are the delta records the ledger
//! prices (Studies carry no Novelty Index by charter; the Fringe
//! chapter has no ledger entry). 35 paper edges + 1 edge to the
//! research work node (the Oracle Progress estimate measures the
//! tracker as a whole). The research → ietf `mentions` class is
//! verified empty (Addendum 7) and stores nothing.
//!
//! The input graph layer is never modified: the output is a copy
//! in which only the Oracle content node's scroll gains lines.

use std::collections::HashMap;
use std::env;
use std::process::ExitCode;

use libphext::phext;
use serde::Deserialize;
use sha2::{Digest, Sha256};

const GRAPH_HOME: &str = "1.1.1/1.3.1/1.1.1";
const CONTENT_PREFIX: &str = "1.1.1/1.3.1/1.2.";
const LEDGER_COORD: &str = "1.1.1/1.1.1/1.1.4";

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
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// The Oracle's registry id: the unit whose source coordinate is
/// the ledger scroll.
fn oracle_id(reg: &Registry) -> Result<u32, String> {
    reg.entries
        .iter()
        .find(|e| e.coordinate == LEDGER_COORD)
        .map(|e| e.id)
        .ok_or_else(|| format!("no registry entry for the ledger {LEDGER_COORD}"))
}

/// Paper delta records: chapter ≥ 2, §1.1 unit, claim opens
/// with "Paper ". Returns registry ids in chapter order.
fn paper_delta_ids(reg: &Registry) -> Vec<u32> {
    let mut chapters: Vec<(u32, u32)> = Vec::new();
    for e in &reg.entries {
        let groups: Vec<&str> = e.coordinate.split('/').collect();
        if groups.len() != 3 {
            continue;
        }
        let ch: Vec<&str> = groups[2].split('.').collect();
        if ch.len() != 3 {
            continue;
        }
        let (chapter, section, scroll) = (
            ch[0].parse::<u32>().unwrap_or(0),
            ch[1].parse::<u32>().unwrap_or(0),
            ch[2].parse::<u32>().unwrap_or(0),
        );
        if chapter >= 2 && section == 1 && scroll == 1 && e.claim.starts_with("Paper ") {
            chapters.push((chapter, e.id));
        }
    }
    chapters.sort();
    chapters.into_iter().map(|(_, id)| id).collect()
}

fn generate(
    graph_text: &str,
    reg: &Registry,
    hop: &str,
) -> Result<(String, usize, u32), String> {
    let oracle = oracle_id(reg)?;
    let papers = paper_delta_ids(reg);
    if papers.is_empty() {
        return Err("paper set computed empty — registry shape changed".into());
    }
    let oracle_coord = format!("{CONTENT_PREFIX}{oracle}");
    let scrolls: HashMap<String, String> = phext::explode(graph_text)
        .iter()
        .map(|(c, s)| (c.to_string(), s.clone()))
        .collect();
    let oracle_scroll = scrolls
        .get(&oracle_coord)
        .ok_or_else(|| format!("graph layer has no Oracle content node at {oracle_coord}"))?
        .clone();
    if !oracle_scroll.starts_with("node · metric ·") {
        return Err(format!(
            "scroll at {oracle_coord} is not the metric node: {}",
            oracle_scroll.lines().next().unwrap_or("")
        ));
    }
    // Every target must exist in the layer before an edge is drawn.
    for id in &papers {
        let target = format!("{CONTENT_PREFIX}{id}");
        if !scrolls.contains_key(&target) {
            return Err(format!("graph layer lacks paper node {target}"));
        }
    }
    if !scrolls.contains_key(GRAPH_HOME) {
        return Err("graph layer lacks its Home node".into());
    }
    let mut addition = String::new();
    addition.push_str(&format!(
        "cross-edges · measures · {} targets · curated from the ledger ({LEDGER_COORD}) · hop {hop}\n",
        papers.len() + 1
    ));
    for id in &papers {
        addition.push_str(&format!(
            "edge · measures · {CONTENT_PREFIX}{id} · w0 · {LEDGER_COORD} · {hop}\n"
        ));
    }
    addition.push_str(&format!(
        "edge · measures · {GRAPH_HOME} · w0 · {LEDGER_COORD} · {hop}\n"
    ));
    let mut out: HashMap<phext::Coordinate, String> = phext::explode(graph_text)
        .iter()
        .map(|(c, s)| (c.clone(), s.clone()))
        .collect();
    let mut new_scroll = oracle_scroll;
    if !new_scroll.ends_with('\n') {
        new_scroll.push('\n');
    }
    new_scroll.push_str(&addition);
    out.insert(phext::to_coordinate(&oracle_coord), new_scroll);
    Ok((phext::implode(out), papers.len() + 1, oracle))
}

/// Fidelity for an edge-adding pass: every scroll byte-identical
/// except the Oracle node's, which must keep its original text as
/// a prefix (lines only appended).
fn fidelity_ok(before_text: &str, after_text: &str, oracle_coord: &str) -> bool {
    let before: HashMap<String, String> = phext::explode(before_text)
        .iter()
        .map(|(c, s)| (c.to_string(), s.clone()))
        .collect();
    let after: HashMap<String, String> = phext::explode(after_text)
        .iter()
        .map(|(c, s)| (c.to_string(), s.clone()))
        .collect();
    if before.len() != after.len() {
        return false;
    }
    for (c, s) in &before {
        match after.get(c) {
            Some(a) if c == oracle_coord => {
                if !a.starts_with(s.as_str()) {
                    return false;
                }
            }
            Some(a) if a == s => {}
            _ => return false,
        }
    }
    true
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 4 || args.len() > 5 {
        eprintln!("usage: kgoat-xedge-gen <research.phext> <registry.json> <research-graph.phext> <out.phext> [hop-stamp]");
        return ExitCode::from(2);
    }
    let hop = args.get(4).map(|s| s.as_str()).unwrap_or("2026-11-23");
    let phext_bytes = match std::fs::read(&args[0]) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("kgoat-xedge-gen: cannot read {}: {e}", args[0]);
            return ExitCode::from(2);
        }
    };
    let reg_text = match std::fs::read_to_string(&args[1]) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("kgoat-xedge-gen: cannot read {}: {e}", args[1]);
            return ExitCode::from(2);
        }
    };
    let reg: Registry = match serde_json::from_str(&reg_text) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("kgoat-xedge-gen: cannot parse {}: {e}", args[1]);
            return ExitCode::from(2);
        }
    };
    let actual = sha256_hex(&phext_bytes);
    if actual != reg.phext_sha256 {
        eprintln!(
            "kgoat-xedge-gen: SHA MISMATCH — registry is for {}, this phext is {actual}; not writing",
            reg.phext_sha256
        );
        return ExitCode::from(1);
    }
    let graph_text = match std::fs::read_to_string(&args[2]) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("kgoat-xedge-gen: cannot read {}: {e}", args[2]);
            return ExitCode::from(2);
        }
    };
    let (out_text, n_edges, oracle) = match generate(&graph_text, &reg, hop) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("kgoat-xedge-gen: {e}; not writing");
            return ExitCode::from(1);
        }
    };
    let oracle_coord = format!("{CONTENT_PREFIX}{oracle}");
    if !fidelity_ok(&graph_text, &out_text, &oracle_coord) {
        eprintln!("kgoat-xedge-gen: FIDELITY FAILURE — a scroll other than the Oracle node's changed; not writing");
        return ExitCode::from(1);
    }
    if let Err(e) = std::fs::write(&args[3], &out_text) {
        eprintln!("kgoat-xedge-gen: cannot write {}: {e}", args[3]);
        return ExitCode::from(2);
    }
    println!(
        "kgoat-xedge-gen: {n_edges} measures edges added on the Oracle node ({oracle_coord}) · sha gate PASS · fidelity PASS → {}",
        args[3]
    );
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn fixture() -> (String, Registry) {
        // A miniature graph layer: Home, the Oracle metric node
        // (id 4), two paper delta nodes (ids 10, 20), one study
        // node (id 30 — must NOT be priced).
        let mut scrolls: HashMap<phext::Coordinate, String> = HashMap::new();
        scrolls.insert(
            phext::to_coordinate(GRAPH_HOME),
            "node · work · research.phext\n".into(),
        );
        scrolls.insert(
            phext::to_coordinate("1.1.1/1.3.1/1.2.4"),
            "node · metric · Research Oracle ledger\nedge · part_of · 1.1.1/1.3.1/1.1.1 · w0 · 1.1.1/1.1.1/1.1.4 · 2026-11-23\n".into(),
        );
        scrolls.insert(
            phext::to_coordinate("1.1.1/1.3.1/1.2.10"),
            "node · work · Paper 001 delta\n".into(),
        );
        scrolls.insert(
            phext::to_coordinate("1.1.1/1.3.1/1.2.20"),
            "node · work · Paper 002 delta\n".into(),
        );
        scrolls.insert(
            phext::to_coordinate("1.1.1/1.3.1/1.2.30"),
            "node · work · Study S001 record\n".into(),
        );
        let reg = Registry {
            phext_sha256: String::new(),
            entries: vec![
                RegEntry { id: 4, coordinate: LEDGER_COORD.into(), claim: "Research Oracle".into() },
                RegEntry { id: 10, coordinate: "1.1.1/1.1.1/2.1.1".into(), claim: "Paper 001 delta: X".into() },
                RegEntry { id: 20, coordinate: "1.1.1/1.1.1/3.1.1".into(), claim: "Paper 002 delta: Y".into() },
                RegEntry { id: 30, coordinate: "1.1.1/1.1.1/7.1.1".into(), claim: "Study S001 source record".into() },
                RegEntry { id: 31, coordinate: "1.1.1/1.1.1/2.1.2".into(), claim: "Paper 001 — Formal core".into() },
            ],
        };
        (phext::implode(scrolls), reg)
    }

    #[test]
    fn measures_edges_hit_papers_and_home_only() {
        let (text, reg) = fixture();
        let (out, n, oracle) = generate(&text, &reg, "2026-11-23").unwrap();
        assert_eq!(oracle, 4);
        assert_eq!(n, 3); // 2 papers + the whole-tracker edge
        assert!(fidelity_ok(&text, &out, "1.1.1/1.3.1/1.2.4"));
        let g = kgoat::builder::build(&[("research".to_string(), out)]);
        assert!(g.violations.is_empty(), "{:?}", g.violations);
        assert_eq!(g.edge_type_counts.get("measures"), Some(&3));
        // The study node received no edge: inbound measures == papers+home only.
        let study_inbound = g
            .inbound
            .get(&("research".to_string(), "1.1.1/1.3.1/1.2.30".to_string()))
            .cloned()
            .unwrap_or_default();
        assert!(study_inbound.iter().all(|(_, ty, _)| ty != "measures"));
    }

    #[test]
    fn paper_set_excludes_studies_and_non_delta_units() {
        let (_, reg) = fixture();
        assert_eq!(paper_delta_ids(&reg), vec![10, 20]);
        let ids: HashSet<u32> = paper_delta_ids(&reg).into_iter().collect();
        assert!(!ids.contains(&30));
        assert!(!ids.contains(&31));
    }
}
