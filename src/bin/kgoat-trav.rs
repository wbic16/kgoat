//! kgoat-trav — the KGOAT traversal service (SPEC §3.1, layer 2), CLI
//! form. The HTTP composition belongs to phext-lattice (Stage 3); the
//! semantics are the graph's own: this binary loads node scrolls with
//! the crate's parser and folds inbound edges live, through the same
//! `builder::build` the index is generated with — there is no second
//! reading of the graph.
//!
//! Usage:
//!   kgoat-trav <home>=<file.phext>... <command> [args]
//!
//! Commands:
//!   node <id>                          the node's page, as text
//!   neighbors <id> [--hops N] [--type T] [--kind K]
//!                  [--min-weight W] [--status S]
//!   path <from-id> <to-id>             shortest path, with derivation
//!   page <id> <out.html>               the §3.2 node page, as HTML
//!   near <coord> [--limit N]           nodes nearest a coordinate
//!
//! An <id> is a coordinate, or <home>@<coordinate> in a federation.

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::env;
use std::process::ExitCode;

use kgoat::builder::{build, Graph, NodeId};
use kgoat::schema::Status;
use kgoat::scroll::Node;

// ---------- small helpers ----------

/// §3.2: statuses in plain words — the visitor never meets a basis code
/// at the door.
fn plain_status(s: Status) -> &'static str {
    match s {
        Status::True => "verified",
        Status::False => "disputed",
        Status::Unknown => "not yet settled",
        Status::Fiction => "a constructed example",
    }
}

/// Weight as strength of language (§3.2); the number lives on the
/// inspect depth (the text page and the derivation lines).
fn plain_weight(w: u8) -> &'static str {
    match w {
        0 => "recorded",
        1 => "connected",
        2 => "strongly connected",
        _ => "defining",
    }
}

fn display(g: &Graph, id: &NodeId) -> String {
    g.nodes
        .get(id)
        .map(|n| n.statement.clone())
        .unwrap_or_else(|| format!("{}@{}", id.0, id.1))
}

fn page_file(id: &NodeId) -> String {
    format!(
        "{}-{}.html",
        id.0,
        id.1.replace('/', "-").replace('.', "_")
    )
}

/// Sentence for an outbound edge, from the source node's page.
fn out_sentence(ty: &str, target_display: &str) -> String {
    match ty {
        "references" => format!("References {target_display}"),
        "supersedes" => format!("Supersedes {target_display}"),
        "implements" => format!("Implements {target_display}"),
        "part_of" => format!("Part of {target_display}"),
        "derived_from" => format!("Derived from {target_display}"),
        "asserts" => format!("Asserts {target_display}"),
        "contradicts" => format!("Contradicts {target_display}"),
        "measures" => format!("Measures {target_display}"),
        "mentions" => format!("Mentions {target_display}"),
        other => format!("{other} {target_display}"),
    }
}

/// Sentence for an inbound edge, from the target node's page.
fn in_sentence(ty: &str, source_display: &str) -> String {
    match ty {
        "references" => format!("Cited by {source_display}"),
        "supersedes" => format!("Superseded by {source_display}"),
        "implements" => format!("Implemented by {source_display}"),
        "part_of" => format!("Has part {source_display}"),
        "derived_from" => format!("Derivation source for {source_display}"),
        "asserts" => format!("Asserted by {source_display}"),
        "contradicts" => format!("Contradicted by {source_display}"),
        "measures" => format!("Measured by {source_display}"),
        "mentions" => format!("Mentioned by {source_display}"),
        other => format!("{other} by {source_display}"),
    }
}

// ---------- the text page (§3.2 content, plain form) ----------

fn render_node_text(g: &Graph, id: &NodeId) -> Option<String> {
    let node = g.nodes.get(id)?;
    let mut out = String::new();
    out.push_str(&format!("{}\n", node.statement));
    out.push_str(&format!("kind: {} · {}@{}\n", node.kind, id.0, id.1));
    if let Some(p) = &node.provenance {
        out.push_str(&format!("provenance: {p}\n"));
    }
    if let Some(a) = &node.assay {
        out.push_str(&format!(
            "status: {} ({}/{}) — receipt: {} · recorded {}\n",
            plain_status(a.status),
            a.status.name(),
            a.basis.name(),
            a.evidence,
            a.hop
        ));
    }
    // Outbound connections as sentences.
    let mut out_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for e in &node.edges {
        *out_counts.entry(e.ty.as_str()).or_default() += 1;
    }
    for (ty, n) in &out_counts {
        out.push_str(&format!("\n{ty} ({n}):\n"));
        for e in node.edges.iter().filter(|e| e.ty == *ty) {
            let target: NodeId = (
                e.target_home.clone().unwrap_or_else(|| id.0.clone()),
                e.target_coord.clone(),
            );
            out.push_str(&format!(
                "  {} · {} · w{}\n",
                out_sentence(ty, &display(g, &target)),
                format!("{}@{}", target.0, target.1),
                e.weight
            ));
        }
    }
    // Inbound connections as sentences (derived, never stored — §2.3).
    if let Some(ins) = g.inbound.get(id) {
        let mut in_counts: BTreeMap<&str, usize> = BTreeMap::new();
        for (_, ty, _) in ins {
            *in_counts.entry(ty.as_str()).or_default() += 1;
        }
        for (ty, n) in &in_counts {
            out.push_str(&format!("\ninbound {ty} ({n}):\n"));
            if ty == &"part_of" && *n > 50 {
                // A whole's page shows counts, not thousands of members.
                let mut by_kind: BTreeMap<String, usize> = BTreeMap::new();
                for (src, _, _) in ins.iter().filter(|(_, t, _)| t.as_str() == *ty) {
                    if let Some(sn) = g.nodes.get(src) {
                        *by_kind.entry(sn.kind.clone()).or_default() += 1;
                    }
                }
                let parts: Vec<String> =
                    by_kind.iter().map(|(k, c)| format!("{c} {k}")).collect();
                out.push_str(&format!("  parts: {}\n", parts.join(" · ")));
                continue;
            }
            for (src, _, w) in ins.iter().filter(|(_, t, _)| t == ty) {
                out.push_str(&format!(
                    "  {} · {}@{} · w{}\n",
                    in_sentence(ty, &display(g, src)),
                    src.0,
                    src.1,
                    w
                ));
            }
        }
    }
    let total_out: usize = out_counts.values().sum();
    let total_in: usize = g.inbound.get(id).map(|v| v.len()).unwrap_or(0);
    out.push_str(&format!(
        "\nstamp: {} connections out · {} in · {} nodes in the fold · {} violations\n",
        total_out,
        total_in,
        g.nodes.len(),
        g.violations.len()
    ));
    Some(out)
}

// ---------- traversal ----------

#[derive(Debug, Clone, Default)]
struct Filters {
    ty: Option<String>,
    kind: Option<String>,
    min_weight: u8,
    status: Option<String>,
}

fn edge_passes(f: &Filters, ty: &str, w: u8) -> bool {
    (f.ty.is_none() || f.ty.as_deref() == Some(ty)) && w >= f.min_weight
}

fn node_passes(g: &Graph, f: &Filters, id: &NodeId) -> bool {
    let Some(n) = g.nodes.get(id) else {
        return false;
    };
    if let Some(k) = &f.kind {
        if &n.kind != k {
            return false;
        }
    }
    if let Some(s) = &f.status {
        match &n.assay {
            Some(a) => {
                if a.status.name() != s {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

/// Breadth-first neighborhood, both directions, hop-limited.
fn neighbors(g: &Graph, start: &NodeId, hops: usize, f: &Filters) -> Vec<(NodeId, usize)> {
    let mut dist: HashMap<NodeId, usize> = HashMap::new();
    let mut queue: VecDeque<NodeId> = VecDeque::new();
    dist.insert(start.clone(), 0);
    queue.push_back(start.clone());
    while let Some(cur) = queue.pop_front() {
        let d = dist[&cur];
        if d >= hops {
            continue;
        }
        let mut next: Vec<NodeId> = Vec::new();
        if let Some(n) = g.nodes.get(&cur) {
            for e in &n.edges {
                if edge_passes(f, &e.ty, e.weight) {
                    next.push((
                        e.target_home.clone().unwrap_or_else(|| cur.0.clone()),
                        e.target_coord.clone(),
                    ));
                }
            }
        }
        if let Some(ins) = g.inbound.get(&cur) {
            for (src, ty, w) in ins {
                if edge_passes(f, ty, *w) {
                    next.push(src.clone());
                }
            }
        }
        for t in next {
            if !dist.contains_key(&t) {
                dist.insert(t.clone(), d + 1);
                queue.push_back(t);
            }
        }
    }
    let mut found: Vec<(NodeId, usize)> = dist
        .into_iter()
        .filter(|(id, d)| *d > 0 && node_passes(g, f, id))
        .collect();
    found.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    found
}

/// One step of a path: the stored edge it rests on, and whether the
/// walk traversed it in its stored direction.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    from: NodeId,
    to: NodeId,
    ty: String,
    weight: u8,
    evidence: String,
    hop: String,
    forward: bool,
}

/// Shortest path between two nodes; the steps are the derivation (§5.3).
fn path(g: &Graph, start: &NodeId, goal: &NodeId) -> Option<Vec<Step>> {
    if start == goal {
        return Some(Vec::new());
    }
    let mut prev: HashMap<NodeId, Step> = HashMap::new();
    let mut seen: BTreeSet<NodeId> = BTreeSet::new();
    let mut queue: VecDeque<NodeId> = VecDeque::new();
    seen.insert(start.clone());
    queue.push_back(start.clone());
    while let Some(cur) = queue.pop_front() {
        // Outbound steps (stored direction).
        if let Some(n) = g.nodes.get(&cur) {
            for e in &n.edges {
                let t: NodeId = (
                    e.target_home.clone().unwrap_or_else(|| cur.0.clone()),
                    e.target_coord.clone(),
                );
                if seen.insert(t.clone()) {
                    prev.insert(
                        t.clone(),
                        Step {
                            from: cur.clone(),
                            to: t.clone(),
                            ty: e.ty.clone(),
                            weight: e.weight,
                            evidence: e.evidence.clone(),
                            hop: e.hop.clone(),
                            forward: true,
                        },
                    );
                    if &t == goal {
                        return Some(unwind(&prev, start, goal));
                    }
                    queue.push_back(t);
                }
            }
        }
        // Inbound steps (walked in reverse; the stored edge is the
        // derivation, the direction is stated, never hidden).
        if let Some(ins) = g.inbound.get(&cur) {
            for (src, ty, w) in ins {
                if seen.insert(src.clone()) {
                    // Recover the stored edge's evidence/hop from the
                    // source node's scroll.
                    let (evidence, hop) = g
                        .nodes
                        .get(src)
                        .and_then(|sn| {
                            sn.edges.iter().find(|e| {
                                &e.ty == ty
                                    && e.target_coord == cur.1
                                    && e.target_home.as_deref() == Some(cur.0.as_str())
                                        || (&e.ty == ty
                                            && e.target_coord == cur.1
                                            && e.target_home.is_none())
                            })
                        })
                        .map(|e| (e.evidence.clone(), e.hop.clone()))
                        .unwrap_or_else(|| ("-".to_string(), "-".to_string()));
                    prev.insert(
                        src.clone(),
                        Step {
                            from: cur.clone(),
                            to: src.clone(),
                            ty: ty.clone(),
                            weight: *w,
                            evidence,
                            hop,
                            forward: false,
                        },
                    );
                    if src == goal {
                        return Some(unwind(&prev, start, goal));
                    }
                    queue.push_back(src.clone());
                }
            }
        }
    }
    None
}

fn unwind(prev: &HashMap<NodeId, Step>, start: &NodeId, goal: &NodeId) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut cur = goal.clone();
    while &cur != start {
        let s = prev[&cur].clone();
        cur = s.from.clone();
        steps.push(s);
    }
    steps.reverse();
    steps
}

fn render_path(g: &Graph, start: &NodeId, goal: &NodeId, steps: &[Step]) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "path {}@{} → {}@{} — {} hop{} (derived; derivation below)\n",
        start.0,
        start.1,
        goal.0,
        goal.1,
        steps.len(),
        if steps.len() == 1 { "" } else { "s" }
    ));
    for (i, s) in steps.iter().enumerate() {
        let (a, b) = if s.forward {
            (&s.from, &s.to)
        } else {
            (&s.to, &s.from)
        };
        out.push_str(&format!(
            "{}. {} —{}→ {} · w{} · evidence {} · {}{}\n",
            i + 1,
            display(g, a),
            s.ty,
            display(g, b),
            s.weight,
            s.evidence,
            s.hop,
            if s.forward {
                String::new()
            } else {
                " · walked in reverse".to_string()
            }
        ));
    }
    out
}

// ---------- holographic neighborhood (§3.1, coordinate half) ----------

fn coord_vec(coord: &str) -> Option<Vec<i64>> {
    let mut v = Vec::new();
    for part in coord.split('/') {
        for n in part.split('.') {
            v.push(n.trim().parse::<i64>().ok()?);
        }
    }
    Some(v)
}

fn near(g: &Graph, home: &str, coord: &str, limit: usize) -> Vec<(NodeId, i64)> {
    let Some(target) = coord_vec(coord) else {
        return Vec::new();
    };
    let mut scored: Vec<(NodeId, i64)> = g
        .nodes
        .keys()
        .filter(|(h, _)| h == home)
        .filter_map(|id| {
            let v = coord_vec(&id.1)?;
            if v.len() != target.len() {
                return None;
            }
            let d: i64 = v.iter().zip(&target).map(|(a, b)| (a - b).abs()).sum();
            Some((id.clone(), d))
        })
        .collect();
    scored.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    scored.truncate(limit);
    scored
}

// ---------- the HTML node page (§3.2 in the §3.3 design language) ----------

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn render_page(g: &Graph, id: &NodeId, stamp: &str) -> Option<String> {
    let node: &Node = g.nodes.get(id)?;
    let mut h = String::new();
    h.push_str("<!DOCTYPE html>\n<html lang=\"en\"><head><meta charset=\"utf-8\">\n");
    h.push_str(&format!("<title>{}</title>\n", esc(&node.statement)));
    h.push_str("<style>\n\
:root{color-scheme:light dark}\n\
body{font-family:Georgia,'Times New Roman',serif;max-width:752px;margin:2rem auto;padding:0 1rem;line-height:1.5}\n\
@media (prefers-color-scheme:dark){body{background:#000;color:#f5f0e6}a{color:#d4af37}}\n\
@media (prefers-color-scheme:light){body{background:#fffdf5;color:#111}a{color:#8a6d1f}}\n\
h1{font-size:1.6rem;border-bottom:2px solid #c9a227;padding-bottom:.4rem}\n\
.tag{display:inline-block;background:#c9a227;color:#000;padding:.05rem .5rem;margin-right:.4rem;font-size:.85rem}\n\
.stamp{font-family:ui-monospace,Menlo,monospace;font-size:.8rem;opacity:.75}\n\
ul{list-style:none;padding-left:0}li{margin:.25rem 0}\n\
</style></head><body>\n");
    h.push_str(&format!("<h1>{}</h1>\n", esc(&node.statement)));
    h.push_str(&format!(
        "<p><span class=\"tag\">{}</span>",
        esc(&node.kind)
    ));
    if let Some(a) = &node.assay {
        h.push_str(&format!(
            "<span class=\"tag\">{}</span>",
            esc(plain_status(a.status))
        ));
    }
    h.push_str("</p>\n");
    if let Some(a) = &node.assay {
        h.push_str(&format!(
            "<p class=\"stamp\">status {} / {} · receipt: {} · recorded {}</p>\n",
            a.status.name(),
            a.basis.name(),
            esc(&a.evidence),
            esc(&a.hop)
        ));
    }
    if let Some(p) = &node.provenance {
        h.push_str(&format!("<p>{}</p>\n", esc(p)));
    }
    // Connections as sentences, each one a door (§3.2).
    let mut sections: Vec<(String, Vec<String>)> = Vec::new();
    let mut by_ty: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for e in &node.edges {
        let target: NodeId = (
            e.target_home.clone().unwrap_or_else(|| id.0.clone()),
            e.target_coord.clone(),
        );
        by_ty.entry(e.ty.as_str()).or_default().push(format!(
            "<li><a href=\"{}\">{}</a> <span class=\"stamp\">{} · w{}</span></li>",
            page_file(&target),
            esc(&out_sentence(&e.ty, &display(g, &target))),
            esc(plain_weight(e.weight)),
            e.weight
        ));
    }
    for (ty, items) in by_ty {
        sections.push((ty.to_string(), items));
    }
    if let Some(ins) = g.inbound.get(id) {
        let mut by_ty_in: BTreeMap<&str, Vec<String>> = BTreeMap::new();
        for (src, ty, w) in ins {
            by_ty_in.entry(ty.as_str()).or_default().push(format!(
                "<li><a href=\"{}\">{}</a> <span class=\"stamp\">{} · w{}</span></li>",
                page_file(src),
                esc(&in_sentence(ty, &display(g, src))),
                esc(plain_weight(*w)),
                w
            ));
        }
        for (ty, items) in by_ty_in {
            if ty == "part_of" && items.len() > 50 {
                sections.push((
                    "parts".to_string(),
                    vec![format!(
                        "<li>{} parts — counted in the stamp, listed on the inspect depth</li>",
                        items.len()
                    )],
                ));
            } else {
                sections.push((format!("inbound {ty}"), items));
            }
        }
    }
    for (title, items) in sections {
        h.push_str(&format!("<h2>{}</h2>\n<ul>\n{}</ul>\n", esc(&title), items.join("\n")));
    }
    let total_in = g.inbound.get(id).map(|v| v.len()).unwrap_or(0);
    h.push_str(&format!(
        "<p class=\"stamp\">{}@{} · {} connections out · {} in · graph folded {} · {} nodes</p>\n",
        id.0,
        id.1,
        node.edges.len(),
        total_in,
        esc(stamp),
        g.nodes.len()
    ));
    h.push_str("</body></html>\n");
    Some(h)
}

// ---------- CLI ----------

fn resolve(first_home: &str, id: &str) -> NodeId {
    match id.split_once('@') {
        Some((home, coord)) => (home.to_string(), coord.to_string()),
        None => (first_home.to_string(), id.to_string()),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut homes: Vec<(String, String)> = Vec::new();
    let mut i = 0usize;
    while i < args.len() && args[i].contains('=') && !args[i].starts_with("--") {
        let (name, path) = args[i].split_once('=').unwrap();
        match std::fs::read_to_string(path) {
            Ok(text) => homes.push((name.to_string(), text)),
            Err(e) => {
                eprintln!("kgoat-trav: cannot read {path}: {e}");
                return ExitCode::from(2);
            }
        }
        i += 1;
    }
    if homes.is_empty() || i >= args.len() {
        eprintln!("usage: kgoat-trav <home>=<file.phext>... <node|neighbors|path|page|near> [args]");
        return ExitCode::from(2);
    }
    let pairs: Vec<(String, String)> = homes
        .iter()
        .map(|(n, t)| (n.clone(), t.clone()))
        .collect();
    let g = build(&pairs);
    if !g.violations.is_empty() {
        eprintln!(
            "kgoat-trav: {} violations in the fold (the scrolls govern; continuing)",
            g.violations.len()
        );
    }
    let first_home = homes[0].0.clone();
    let cmd = args[i].as_str();
    let rest = &args[i + 1..];

    match cmd {
        "node" if rest.len() == 1 => {
            let id = resolve(&first_home, &rest[0]);
            match render_node_text(&g, &id) {
                Some(text) => {
                    print!("{text}");
                    ExitCode::SUCCESS
                }
                None => {
                    eprintln!("kgoat-trav: no node at {}@{}", id.0, id.1);
                    ExitCode::from(1)
                }
            }
        }
        "neighbors" if !rest.is_empty() => {
            let id = resolve(&first_home, &rest[0]);
            let mut hops = 1usize;
            let mut f = Filters::default();
            let mut j = 1usize;
            while j < rest.len() {
                match rest[j].as_str() {
                    "--hops" => {
                        hops = rest.get(j + 1).and_then(|v| v.parse().ok()).unwrap_or(1);
                        j += 2;
                    }
                    "--type" => {
                        f.ty = rest.get(j + 1).cloned();
                        j += 2;
                    }
                    "--kind" => {
                        f.kind = rest.get(j + 1).cloned();
                        j += 2;
                    }
                    "--min-weight" => {
                        f.min_weight =
                            rest.get(j + 1).and_then(|v| v.parse().ok()).unwrap_or(0);
                        j += 2;
                    }
                    "--status" => {
                        f.status = rest.get(j + 1).cloned();
                        j += 2;
                    }
                    other => {
                        eprintln!("kgoat-trav: unknown option {other}");
                        return ExitCode::from(2);
                    }
                }
            }
            if !g.nodes.contains_key(&id) {
                eprintln!("kgoat-trav: no node at {}@{}", id.0, id.1);
                return ExitCode::from(1);
            }
            let found = neighbors(&g, &id, hops, &f);
            for (n, d) in &found {
                println!(
                    "hop {} · {} · {}@{}",
                    d,
                    display(&g, n),
                    n.0,
                    n.1
                );
            }
            println!("stamp: {} neighbors within {} hop(s)", found.len(), hops);
            ExitCode::SUCCESS
        }
        "path" if rest.len() == 2 => {
            let a = resolve(&first_home, &rest[0]);
            let b = resolve(&first_home, &rest[1]);
            match path(&g, &a, &b) {
                Some(steps) => {
                    print!("{}", render_path(&g, &a, &b, &steps));
                    ExitCode::SUCCESS
                }
                None => {
                    println!("no path {}@{} → {}@{}", a.0, a.1, b.0, b.1);
                    ExitCode::from(1)
                }
            }
        }
        "page" if rest.len() == 2 => {
            let id = resolve(&first_home, &rest[0]);
            let stamp = chrono_stamp();
            match render_page(&g, &id, &stamp) {
                Some(html) => match std::fs::write(&rest[1], html) {
                    Ok(()) => {
                        println!("page {}@{} → {}", id.0, id.1, rest[1]);
                        ExitCode::SUCCESS
                    }
                    Err(e) => {
                        eprintln!("kgoat-trav: cannot write {}: {e}", rest[1]);
                        ExitCode::from(2)
                    }
                },
                None => {
                    eprintln!("kgoat-trav: no node at {}@{}", id.0, id.1);
                    ExitCode::from(1)
                }
            }
        }
        "near" if !rest.is_empty() => {
            let (home, coord) = match rest[0].split_once('@') {
                Some((h, c)) => (h.to_string(), c.to_string()),
                None => (first_home.clone(), rest[0].clone()),
            };
            let mut limit = 10usize;
            let mut j = 1usize;
            while j < rest.len() {
                if rest[j] == "--limit" {
                    limit = rest.get(j + 1).and_then(|v| v.parse().ok()).unwrap_or(10);
                    j += 2;
                } else {
                    eprintln!("kgoat-trav: unknown option {}", rest[j]);
                    return ExitCode::from(2);
                }
            }
            for (id, d) in near(&g, &home, &coord, limit) {
                println!("distance {} · {} · {}@{}", d, display(&g, &id), id.0, id.1);
            }
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("usage: kgoat-trav <home>=<file.phext>... <node|neighbors|path|page|near> [args]");
            ExitCode::from(2)
        }
    }
}

/// The fold stamp: an ISO-ish local stamp without a clock dependency —
/// the crate carries no datetime dependency, so the stamp is the
/// caller's date when supplied via KGOAT_STAMP, else "unstamped".
fn chrono_stamp() -> String {
    env::var("KGOAT_STAMP").unwrap_or_else(|_| "unstamped".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use libphext::phext;
    use std::collections::HashMap;

    fn fixture() -> Graph {
        let mut scrolls: HashMap<phext::Coordinate, String> = HashMap::new();
        let put = |scrolls: &mut HashMap<phext::Coordinate, String>, c: &str, t: &str| {
            scrolls.insert(phext::to_coordinate(c), t.to_string());
        };
        put(
            &mut scrolls,
            "1.1.1/1.1.1/1.1.1",
            "node · work · The Test Corpus\ndocument count: 2\n",
        );
        put(
            &mut scrolls,
            "1.1.1/1.1.1/1.2.1",
            "node · document · DOC 1 — First\ndocument\n\
             edge · references · 1.1.1/1.1.1/1.2.2 · w0 · 1.1.1/1.1.1/1.2.1 · 2026-11-23\n\
             edge · part_of · 1.1.1/1.1.1/1.1.1 · w0 · 1.1.1/1.1.1/1.2.1 · 2026-11-23\n",
        );
        put(
            &mut scrolls,
            "1.1.1/1.1.1/1.2.2",
            "node · document · DOC 2 — Second\ndocument\n\
             edge · supersedes · 1.1.1/1.1.1/1.2.1 · w0 · 1.1.1/1.1.1/1.2.2 · 2026-11-23\n\
             edge · part_of · 1.1.1/1.1.1/1.1.1 · w0 · 1.1.1/1.1.1/1.2.2 · 2026-11-23\n",
        );
        put(
            &mut scrolls,
            "1.1.1/1.1.1/1.3.1",
            "node · project · Widget\na project\n\
             edge · implements · 1.1.1/1.1.1/1.2.1 · w0 · 1.1.1/1.1.1/1.3.1 · 2026-11-23\n\
             edge · part_of · 1.1.1/1.1.1/1.1.1 · w0 · 1.1.1/1.1.1/1.3.1 · 2026-11-23\n",
        );
        put(
            &mut scrolls,
            "1.1.1/1.1.1/1.4.1",
            "node · claim · DOC 1 record — existence of the publication\n\
             assay · True/VERIFIED · 1.1.1/1.1.1/1.2.1 · 2026-11-23\n\
             edge · part_of · 1.1.1/1.1.1/1.1.1 · w0 · 1.1.1/1.1.1/1.4.1 · 2026-11-23\n",
        );
        let text = phext::implode(scrolls);
        build(&[("test".to_string(), text)])
    }

    fn id(coord: &str) -> NodeId {
        ("test".to_string(), coord.to_string())
    }

    #[test]
    fn node_page_renders_sentences_and_plain_status() {
        let g = fixture();
        assert!(g.violations.is_empty(), "{:?}", g.violations);
        let page = render_node_text(&g, &id("1.1.1/1.1.1/1.2.1")).unwrap();
        assert!(page.contains("References DOC 2 — Second"), "{page}");
        assert!(page.contains("Superseded by DOC 2 — Second"), "{page}");
        assert!(page.contains("Implemented by Widget"), "{page}");
        let claim = render_node_text(&g, &id("1.1.1/1.1.1/1.4.1")).unwrap();
        assert!(claim.contains("status: verified (True/VERIFIED)"), "{claim}");
    }

    #[test]
    fn neighbors_respect_type_filter() {
        let g = fixture();
        let mut f = Filters::default();
        f.ty = Some("references".to_string());
        let found = neighbors(&g, &id("1.1.1/1.1.1/1.2.1"), 1, &f);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].0, id("1.1.1/1.1.1/1.2.2"));
        // Unfiltered: references target (DOC 2), part_of whole, and the
        // implementing project = 3 distinct nodes at one hop (DOC 2 is
        // also the superseding source — one node, counted once).
        let all = neighbors(&g, &id("1.1.1/1.1.1/1.2.1"), 1, &Filters::default());
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn path_carries_its_derivation() {
        let g = fixture();
        // Widget → DOC 1 (implements) → DOC 2 (references): 2 hops.
        let steps = path(&g, &id("1.1.1/1.1.1/1.3.1"), &id("1.1.1/1.1.1/1.2.2")).unwrap();
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0].ty, "implements");
        assert_eq!(steps[1].ty, "references");
        let text = render_path(&g, &id("1.1.1/1.1.1/1.3.1"), &id("1.1.1/1.1.1/1.2.2"), &steps);
        assert!(text.contains("derivation below"), "{text}");
        assert!(text.contains("evidence"), "{text}");
    }

    #[test]
    fn near_ranks_by_coordinate_distance() {
        let g = fixture();
        let found = near(&g, "test", "1.1.1/1.1.1/1.2.9", 2);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].0, id("1.1.1/1.1.1/1.2.2"));
        assert_eq!(found[0].1, 7);
    }

    #[test]
    fn html_page_has_design_stamps() {
        let g = fixture();
        let html = render_page(&g, &id("1.1.1/1.1.1/1.2.1"), "2026-11-23").unwrap();
        assert!(html.contains("max-width:752px"), "measure");
        assert!(html.contains("Cited by") || html.contains("Superseded by"));
        assert!(html.contains("test@1.1.1/1.1.1/1.2.1"), "footer stamp: {html}");
    }
}
