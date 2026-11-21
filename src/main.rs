//! kgoat-build — the KGOAT index builder CLI (SPEC §2.3).
//!
//! Usage: `kgoat-build <home>=<file.phext> [<home>=<file.phext> ...] --out <index.phext>`
//!
//! Reads the node scrolls of each home phext, folds them into the derived
//! surfaces (inbound-edge index, kind/type tables, assay registry), writes
//! the generated index phext, and prints a summary. Exit status is 1 when
//! the graph carries schema violations — the scrolls are right, but a
//! non-conformant graph is reported as one.

use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

/// Format a Unix timestamp (seconds) as an ISO datetime in UTC, without a
/// date library: the civil-from-days algorithm, in the open.
fn iso_stamp(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    let (hh, mm, ss) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut homes: Vec<(String, String)> = Vec::new();
    let mut out: Option<String> = None;
    let mut stamp: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                out = args.get(i).cloned();
            }
            "--stamp" => {
                i += 1;
                stamp = args.get(i).cloned();
            }
            other => match other.split_once('=') {
                Some((home, path)) => match std::fs::read_to_string(path) {
                    Ok(text) => homes.push((home.to_string(), text)),
                    Err(e) => {
                        eprintln!("kgoat-build: cannot read {path}: {e}");
                        return ExitCode::from(2);
                    }
                },
                None => {
                    eprintln!("kgoat-build: bad argument `{other}` (want <home>=<file.phext>)");
                    return ExitCode::from(2);
                }
            },
        }
        i += 1;
    }
    let out = match out {
        Some(o) => o,
        None => {
            eprintln!("kgoat-build: --out <index.phext> is required");
            return ExitCode::from(2);
        }
    };
    if homes.is_empty() {
        eprintln!("kgoat-build: at least one <home>=<file.phext> is required");
        return ExitCode::from(2);
    }
    let stamp = stamp.unwrap_or_else(|| {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        iso_stamp(secs)
    });

    let g = kgoat::builder::build(&homes);
    let index = kgoat::builder::render_index(&g, &stamp);
    if let Err(e) = std::fs::write(&out, index) {
        eprintln!("kgoat-build: cannot write {out}: {e}");
        return ExitCode::from(2);
    }
    let total_edges: usize = g.edge_type_counts.values().sum();
    println!(
        "kgoat-build: {} homes · {} nodes · {} edges · {} registry entries · {} violations → {out}",
        g.homes.len(),
        g.nodes.len(),
        total_edges,
        g.registry.len(),
        g.violations.len()
    );
    for v in &g.violations {
        eprintln!("violation: {v}");
    }
    if g.violations.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
