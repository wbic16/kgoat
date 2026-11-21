//! The builder (SPEC §2.3): reads every node scroll across a federation of
//! phexts and generates the derived surfaces — the inbound-edge index, the
//! kind/type tables, and the assay registry — plus a conformance report.
//!
//! The index is generated, never maintained. Every rollup is calculated
//! from the stored atoms, never hand-written. If the index and a scroll
//! disagree, the scroll is right and the index is rebuilt.

use std::collections::BTreeMap;
use std::collections::HashMap;

use libphext::phext;

use crate::schema::Status;
use crate::scroll::{parse_node, Node};

/// A node's identity: (home phext, coordinate) — nothing else (SPEC §2.1).
pub type NodeId = (String, String);

/// Per-home input facts, carried into the generated Home scroll.
#[derive(Debug, Clone)]
pub struct HomeInfo {
    pub name: String,
    pub scrolls: usize,
    pub node_scrolls: usize,
    /// The home phext's hash, by libphext's own checksum.
    pub checksum: String,
}

/// One assay-registry entry (SPEC §5.1): one per claim-bearing unit.
#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub unit: String,
    pub claim: String,
    pub status: Status,
    pub basis: String,
    pub evidence: String,
}

/// The folded graph: parsed nodes, schema violations, and the derived
/// surfaces, all calculated from the scrolls.
#[derive(Debug, Default)]
pub struct Graph {
    pub homes: Vec<HomeInfo>,
    pub nodes: BTreeMap<NodeId, Node>,
    pub violations: Vec<String>,
    /// target → (source, type, weight): the reverse traversal the stored
    /// form deliberately omits (SPEC §2.3).
    pub inbound: BTreeMap<NodeId, Vec<(NodeId, String, u8)>>,
    pub kind_counts: BTreeMap<String, usize>,
    pub edge_type_counts: BTreeMap<String, usize>,
    pub status_counts: BTreeMap<String, usize>,
    pub registry: Vec<RegistryEntry>,
    /// Edges whose target is not a node scroll in the federation. Counted,
    /// not condemned: an edge may honestly point at material that has no
    /// node scroll yet.
    pub edges_to_non_nodes: usize,
}

fn unit_id(home: &str, coord: &str) -> String {
    format!("{home}@{coord}")
}

/// Fold a federation of phexts into a graph. `homes` is (home name, phext
/// text) pairs. A scroll is a node scroll iff its first content line opens
/// with `node · `; every other scroll is material the graph sits beside
/// (SPEC §2.5), counted but not parsed.
pub fn build(homes: &[(String, String)]) -> Graph {
    let mut g = Graph::default();
    for (home, text) in homes {
        let scrolls = phext::explode(text);
        let mut sorted: Vec<(String, String)> = scrolls
            .iter()
            .map(|(c, s)| (c.to_string(), s.clone()))
            .collect();
        sorted.sort();
        let mut node_scrolls = 0usize;
        for (coord, scroll) in &sorted {
            let first = scroll.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
            if !first.trim_start().starts_with("node · ") && !first.trim_start().starts_with("node ·") {
                continue;
            }
            node_scrolls += 1;
            match parse_node(coord, scroll) {
                Ok(node) => {
                    g.nodes.insert((home.clone(), coord.clone()), node);
                }
                Err(errs) => {
                    for e in errs {
                        g.violations
                            .push(format!("{}: {e}", unit_id(home, coord)));
                    }
                }
            }
        }
        g.homes.push(HomeInfo {
            name: home.clone(),
            scrolls: scrolls.len(),
            node_scrolls,
            checksum: phext::checksum(text),
        });
    }

    // Derived surfaces, folded from the stored atoms.
    for ((home, coord), node) in &g.nodes {
        *g.kind_counts.entry(node.kind.clone()).or_default() += 1;
        if let Some(a) = &node.assay {
            *g.status_counts.entry(a.status.name().to_string()).or_default() += 1;
            g.registry.push(RegistryEntry {
                unit: unit_id(home, coord),
                claim: node.statement.clone(),
                status: a.status,
                basis: a.basis.name().to_string(),
                evidence: a.evidence.clone(),
            });
        }
        for (i, e) in node.edges.iter().enumerate() {
            *g.edge_type_counts.entry(e.ty.clone()).or_default() += 1;
            let target: NodeId = (
                e.target_home.clone().unwrap_or_else(|| home.clone()),
                e.target_coord.clone(),
            );
            if g.nodes.contains_key(&target) {
                g.inbound
                    .entry(target)
                    .or_default()
                    .push(((home.clone(), coord.clone()), e.ty.clone(), e.weight));
            } else {
                g.edges_to_non_nodes += 1;
            }
            if let Some(a) = &e.assay {
                *g.status_counts.entry(a.status.name().to_string()).or_default() += 1;
                g.registry.push(RegistryEntry {
                    unit: format!("{} edge#{}", unit_id(home, coord), i + 1),
                    claim: format!("{} — {} → {}", node.statement, e.ty, e.target_coord),
                    status: a.status,
                    basis: a.basis.name().to_string(),
                    evidence: a.evidence.clone(),
                });
            }
        }
    }
    g.registry.sort_by(|a, b| a.unit.cmp(&b.unit));
    g
}

fn counts_line(label: &str, counts: &BTreeMap<String, usize>) -> String {
    if counts.is_empty() {
        return format!("{label}: none");
    }
    let parts: Vec<String> = counts.iter().map(|(k, v)| format!("{k} {v}")).collect();
    format!("{label}: {}", parts.join(" · "))
}

/// Render the generated index as a phext (SPEC §2.3): a Home scroll with
/// the rollup and the builder's stamp, and one scroll per derived surface.
/// `stamp` is the build's hop stamp (an ISO datetime string).
pub fn render_index(g: &Graph, stamp: &str) -> String {
    let total_edges: usize = g.edge_type_counts.values().sum();
    let mut home = String::new();
    home.push_str(&format!(
        "KGOAT index · generated by kgoat-build {} · {stamp}\n",
        env!("CARGO_PKG_VERSION")
    ));
    home.push_str("The scrolls are right; this index is a fold over them (SPEC §2.3).\n\n");
    home.push_str("homes:\n");
    for h in &g.homes {
        home.push_str(&format!(
            "- {} · {} scrolls · {} node scrolls · checksum {}\n",
            h.name, h.scrolls, h.node_scrolls, h.checksum
        ));
    }
    home.push_str(&format!(
        "\nnodes: {} · edges: {} · edges to non-node targets: {}\n",
        g.nodes.len(),
        total_edges,
        g.edges_to_non_nodes
    ));
    home.push_str(&format!("{}\n", counts_line("kinds", &g.kind_counts)));
    home.push_str(&format!("{}\n", counts_line("statuses", &g.status_counts)));
    home.push_str(&format!("violations: {}\n", g.violations.len()));

    let mut inbound = String::from("inbound edges — derived, never stored (SPEC §2.3)\n");
    if g.inbound.is_empty() {
        inbound.push_str("none\n");
    }
    for (target, sources) in &g.inbound {
        for (source, ty, w) in sources {
            inbound.push_str(&format!(
                "{} ← {} · {} · w{}\n",
                unit_id(&target.0, &target.1),
                unit_id(&source.0, &source.1),
                ty,
                w
            ));
        }
    }

    let mut tables = String::from("kind and edge-type tables — counted from the scrolls\n");
    tables.push_str(&format!("{}\n", counts_line("kinds", &g.kind_counts)));
    tables.push_str(&format!("{}\n", counts_line("edge types", &g.edge_type_counts)));
    tables.push_str(&format!("{}\n", counts_line("statuses", &g.status_counts)));

    let mut registry = String::from("assay registry — one entry per claim-bearing unit (SPEC §5.1)\n");
    if g.registry.is_empty() {
        registry.push_str("none\n");
    }
    for r in &g.registry {
        registry.push_str(&format!(
            "{} · {} · {}/{} · {}\n",
            r.unit,
            r.claim,
            r.status.name(),
            r.basis,
            r.evidence
        ));
    }

    let mut conformance = String::from("conformance — schema violations found by the builder\n");
    if g.violations.is_empty() {
        conformance.push_str("none — the graph conforms to the format\n");
    }
    for v in &g.violations {
        conformance.push_str(&format!("- {v}\n"));
    }

    let mut map: HashMap<phext::Coordinate, String> = HashMap::new();
    map.insert(phext::to_coordinate("1.1.1/1.1.1/1.1.1"), home);
    map.insert(phext::to_coordinate("1.1.1/1.1.1/1.1.2"), inbound);
    map.insert(phext::to_coordinate("1.1.1/1.1.1/1.1.3"), tables);
    map.insert(phext::to_coordinate("1.1.1/1.1.1/1.1.4"), registry);
    map.insert(phext::to_coordinate("1.1.1/1.1.1/1.1.5"), conformance);
    phext::implode(map)
}
