//! End-to-end tests for the KGOAT schema, parser, and builder, against
//! phexts constructed with libphext itself — material this crate did not
//! write by hand (the conformance habit: check against another tool's
//! files, here the reference implementation's own implode/explode).

use std::collections::HashMap;

use kgoat::builder;
use kgoat::schema::{self, Basis, Status};
use kgoat::scroll::parse_node;
use libphext::phext;

fn phext_of(scrolls: &[(&str, &str)]) -> String {
    let mut map: HashMap<phext::Coordinate, String> = HashMap::new();
    for (coord, text) in scrolls {
        map.insert(phext::to_coordinate(coord), text.to_string());
    }
    phext::implode(map)
}

const DOC: &str = "node · document · RFC 9110 — HTTP Semantics\n\
                   provenance · IETF, edited by Fielding et al. (attribution)\n";

const PROJECT: &str = "node · project · rfcbox — the RFC Box programme\n\
                       edge · implements · 1.1.1/1.1.1/1.1.1 · w3 · ietf.phext build record · 2026-11-18\n\
                       edge · mentions · 1.1.1/1.1.1/1.1.9 · w1 · corpus audit · 2026-11-16\n";

const CLAIM: &str = "node · claim · a single plain phext file can carry the whole graph\n\
                     assay · Unknown/ATTRIBUTED · boardy-boot-manual.md · 2026-11-18\n\
                     edge · derived_from · 1.1.1/1.1.1/1.1.1 · w2 · SPEC §4.1 · 2026-11-18\n\
                     history · 2026-11-18 · Unknown->Fiction · drafting note\n\
                     history · 2026-11-19 · Fiction->Unknown · assay review\n";

const OTHER_CLAIM: &str = "node · claim · the graph needs a database underneath\n\
                           assay · False/REFUTED · holo-bench RESULTS.md · 2026-11-16\n\
                           edge · contradicts · ietf.phext@1.1.1/1.1.1/1.1.3 · w2 · False/REFUTED · holo-bench RESULTS.md · 2026-11-16\n";

#[test]
fn schema_tables_match_the_spec() {
    // Eight kinds, and no person kind — KGOAT is for ideas, not people (D6).
    assert_eq!(schema::NODE_KINDS.len(), 8);
    assert!(!schema::is_node_kind("person"));
    // `made` is attribution, never an edge type.
    assert!(!schema::is_edge_type("made"));
    assert_eq!(schema::EDGE_TYPES.len(), 9);
    // mentions is capped at weight 1; everything else at 3.
    assert_eq!(schema::max_weight("mentions"), 1);
    assert_eq!(schema::max_weight("references"), 3);
}

#[test]
fn basis_constrains_status() {
    assert!(schema::status_legal_for_basis(Status::True, Basis::Verified));
    assert!(schema::status_legal_for_basis(Status::True, Basis::Stipulation));
    assert!(!schema::status_legal_for_basis(Status::True, Basis::Attributed));
    assert!(!schema::status_legal_for_basis(Status::False, Basis::Verified));
    assert!(schema::status_legal_for_basis(Status::False, Basis::Refuted));
    assert!(schema::status_legal_for_basis(Status::Fiction, Basis::Construction));
    assert!(!schema::status_legal_for_basis(Status::True, Basis::Construction));
}

#[test]
fn transitions_follow_the_mvo_edges() {
    assert!(schema::transition_legal(Status::Unknown, Status::True));
    assert!(schema::transition_legal(Status::True, Status::False));
    assert!(schema::transition_legal(Status::Fiction, Status::Unknown));
    // Fiction never goes directly to True; there is no silent promotion.
    assert!(!schema::transition_legal(Status::Fiction, Status::True));
    assert!(!schema::transition_legal(Status::False, Status::True));
}

#[test]
fn parses_a_full_node_scroll() {
    let n = parse_node("1.1.1/1.1.1/1.1.3", CLAIM).expect("claim scroll parses");
    assert_eq!(n.kind, "claim");
    assert_eq!(n.edges.len(), 1);
    assert_eq!(n.edges[0].ty, "derived_from");
    assert_eq!(n.history.len(), 2);
    let a = n.assay.expect("claim carries its assay");
    assert_eq!(a.status, Status::Unknown);
    assert_eq!(a.basis, Basis::Attributed);
}

#[test]
fn rejects_nonconformant_scrolls() {
    // A person node is non-conformant by scope (D6).
    let person = "node · person · Someone\n";
    assert!(parse_node("1.1.1/1.1.1/1.1.1", person).is_err());
    // mentions above its weight cap.
    let fat_mention = "node · work · A thing\nedge · mentions · 1.1.1/1.1.1/1.1.2 · w2 · x · 2026-11-21\n";
    assert!(parse_node("1.1.1/1.1.1/1.1.1", fat_mention).is_err());
    // A claim-bearing edge without its status/basis pair.
    let bare_assert = "node · org · An org\nedge · asserts · 1.1.1/1.1.1/1.1.3 · w2 · x · 2026-11-21\n";
    assert!(parse_node("1.1.1/1.1.1/1.1.1", bare_assert).is_err());
    // A structural edge decorated with an assay it does not need.
    let dressed = "node · work · A thing\nedge · part_of · 1.1.1/1.1.1/1.1.2 · w2 · True/VERIFIED · x · 2026-11-21\n";
    assert!(parse_node("1.1.1/1.1.1/1.1.1", dressed).is_err());
    // `made` written as an edge.
    let made = "node · work · A thing\nedge · made · 1.1.1/1.1.1/1.1.2 · w3 · x · 2026-11-21\n";
    assert!(parse_node("1.1.1/1.1.1/1.1.1", made).is_err());
    // An illegal status transition in history.
    let promoted = "node · claim · A claim\nassay · True/VERIFIED · receipt · 2026-11-21\nhistory · 2026-11-20 · Fiction->True · note\n";
    assert!(parse_node("1.1.1/1.1.1/1.1.1", promoted).is_err());
}

#[test]
fn builder_folds_the_federation() {
    let home_a = phext_of(&[
        ("1.1.1/1.1.1/1.1.1", DOC),
        ("1.1.1/1.1.1/1.1.2", PROJECT),
        ("1.1.1/1.1.1/1.1.3", CLAIM),
    ]);
    let home_b = phext_of(&[("1.1.1/1.1.1/1.1.1", OTHER_CLAIM)]);
    let g = builder::build(&[
        ("ietf.phext".to_string(), home_a),
        ("research.phext".to_string(), home_b),
    ]);
    assert!(g.violations.is_empty(), "violations: {:?}", g.violations);
    assert_eq!(g.nodes.len(), 4);
    assert_eq!(g.kind_counts.get("claim"), Some(&2));
    assert_eq!(g.kind_counts.get("document"), Some(&1));
    // The registry holds both claim nodes and the one claim-bearing edge.
    assert_eq!(g.registry.len(), 3);
    assert_eq!(g.status_counts.get("False"), Some(&2));
    // The inbound index derives the reverse of `implements`.
    let doc_id = ("ietf.phext".to_string(), "1.1.1/1.1.1/1.1.1".to_string());
    let inbound = g.inbound.get(&doc_id).expect("doc has inbound edges");
    assert!(inbound
        .iter()
        .any(|(src, ty, w)| ty == "implements" && *w == 3 && src.1 == "1.1.1/1.1.1/1.1.2"));
    // The PROJECT mentions target has no node scroll: counted, not condemned.
    assert_eq!(g.edges_to_non_nodes, 1);

    // The rendered index is itself a phext, readable by libphext alone.
    let index = builder::render_index(&g, "2026-11-21T16:09:00Z");
    let scrolls = phext::explode(&index);
    let home_scroll = scrolls
        .get(&phext::to_coordinate("1.1.1/1.1.1/1.1.1"))
        .expect("index has a Home scroll");
    assert!(home_scroll.contains("nodes: 4"));
    assert!(home_scroll.contains("violations: 0"));
}

#[test]
fn builder_reports_a_person_node_as_a_violation() {
    let bad = phext_of(&[("1.1.1/1.1.1/1.1.1", "node · person · Someone\n")]);
    let g = builder::build(&[("x.phext".to_string(), bad)]);
    assert_eq!(g.nodes.len(), 0);
    assert_eq!(g.violations.len(), 1);
    assert!(g.violations[0].contains("not people"));
}
