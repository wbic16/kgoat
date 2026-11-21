//! The node-scroll parser (SPEC §§1.2, 2.2, 2.4). A node is a scroll: a
//! page of plain text at a coordinate in a phext. Read top to bottom, a
//! node scroll is what it is (kind + plain-language statement), its assay
//! record if it bears claims, its outbound edges, and its history — every
//! scroll reads as a meaningful unit in a plain text editor.
//!
//! Line grammar (fields separated by ` · `, no brackets, no tags):
//!
//! ```text
//! node · <kind> · <one-line statement of the thing>
//! provenance · <attribution text>                      (optional)
//! assay · <Status>/<BASIS> · <evidence pointer> · <hop stamp>   (claim nodes only)
//! edge · <type> · <target> · w<0-3> · <evidence pointer> · <hop stamp>
//! edge · <type> · <target> · w<0-3> · <Status>/<BASIS> · <evidence pointer> · <hop stamp>
//! history · <hop stamp> · <From>-><To> · <evidence pointer>
//! ```
//!
//! `<target>` is a coordinate in the same phext, or `<home>@<coordinate>`
//! for a node in another federated phext (SPEC §2.5). Any other line is
//! body text: part of the page, not part of the graph's stored form.

use crate::schema::{self, Basis, Status};

/// A status/basis/evidence triple — the assay record of a claim-bearing
/// unit (SPEC §5.1) — with the hop stamp of when the graph recorded it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assay {
    pub status: Status,
    pub basis: Basis,
    pub evidence: String,
    pub hop: String,
}

/// One stored outbound edge (SPEC §2.2). Edges are stored exactly once, in
/// the source node's scroll; inbound edges are derived by the builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub ty: String,
    /// Home phext of the target; `None` means the source's own phext.
    pub target_home: Option<String>,
    pub target_coord: String,
    pub weight: u8,
    /// Present only on claim-bearing edges (SPEC §2.2).
    pub assay: Option<Assay>,
    pub evidence: String,
    pub hop: String,
}

/// A recorded status transition (SPEC §2.4). History is kept, not
/// overwritten; the present is the builder's fold over it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    pub hop: String,
    pub from: Status,
    pub to: Status,
    pub evidence: String,
}

/// A parsed node scroll. Identity is placement: `coord` is the scroll's
/// coordinate inside its home phext, supplied by the caller (SPEC §2.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub coord: String,
    pub kind: String,
    pub statement: String,
    /// Attribution (e.g. a maker's name in provenance lines). Attribution is
    /// a statement about the work, never a node (SPEC §1.2, D6).
    pub provenance: Option<String>,
    /// Exactly one assay record for claim nodes; none for other kinds.
    pub assay: Option<Assay>,
    pub edges: Vec<Edge>,
    pub history: Vec<Transition>,
}

fn parse_assay_pair(field: &str) -> Result<(Status, Basis), String> {
    let (s, b) = field
        .split_once('/')
        .ok_or_else(|| format!("assay pair `{field}` is not <Status>/<BASIS>"))?;
    let status = Status::parse(s).ok_or_else(|| format!("unknown status `{s}`"))?;
    let basis = Basis::parse(b).ok_or_else(|| format!("unknown basis `{b}`"))?;
    if !schema::status_legal_for_basis(status, basis) {
        return Err(format!(
            "status {} is not legal for basis {} (SPEC §5.1)",
            status.name(),
            basis.name()
        ));
    }
    Ok((status, basis))
}

fn parse_edge(tokens: &[&str], line: &str) -> Result<Edge, String> {
    // tokens[0] == "edge"; total 6 (structural) or 7 (claim-bearing).
    if tokens.len() != 6 && tokens.len() != 7 {
        return Err(format!(
            "edge line has {} fields, want 6 or 7: `{line}`",
            tokens.len()
        ));
    }
    let ty = tokens[1];
    if ty == "made" {
        return Err(
            "`made` is attribution on the work's scroll, not an edge (SPEC §1.2)".to_string(),
        );
    }
    if !schema::is_edge_type(ty) {
        return Err(format!("unknown edge type `{ty}`"));
    }
    let (target_home, target_coord) = match tokens[2].split_once('@') {
        Some((home, coord)) => (Some(home.to_string()), coord.to_string()),
        None => (None, tokens[2].to_string()),
    };
    if !schema::coordinate_well_formed(&target_coord) {
        return Err(format!("edge target `{}` is not a well-formed coordinate", tokens[2]));
    }
    let weight: u8 = tokens[3]
        .strip_prefix('w')
        .and_then(|n| n.parse::<u8>().ok())
        .ok_or_else(|| format!("edge weight `{}` is not w<0-3>", tokens[3]))?;
    if weight > schema::max_weight(ty) {
        return Err(format!(
            "edge type `{ty}` has weight {weight}, above its cap of {} (SPEC §1.3)",
            schema::max_weight(ty)
        ));
    }
    let (assay, evidence, hop) = if tokens.len() == 7 {
        if !schema::assay_permitted(ty) {
            return Err(format!(
                "structural edge type `{ty}` must not carry a status/basis pair (SPEC §2.2)"
            ));
        }
        let (status, basis) = parse_assay_pair(tokens[4])?;
        (
            Some(Assay {
                status,
                basis,
                evidence: tokens[5].to_string(),
                hop: tokens[6].to_string(),
            }),
            tokens[5].to_string(),
            tokens[6].to_string(),
        )
    } else {
        if schema::assay_required(ty) {
            return Err(format!(
                "claim-bearing edge type `{ty}` must carry a status/basis pair and evidence (SPEC §5.1)"
            ));
        }
        (None, tokens[4].to_string(), tokens[5].to_string())
    };
    if evidence.is_empty() {
        return Err("edge evidence pointer is empty".to_string());
    }
    if assay.is_some() && evidence == "-" {
        return Err("claim-bearing edge has no evidence pointer (SPEC §5.1)".to_string());
    }
    if !schema::hop_stamp_well_formed(&hop) {
        return Err(format!("hop stamp `{hop}` is not an ISO date"));
    }
    Ok(Edge {
        ty: ty.to_string(),
        target_home,
        target_coord,
        weight,
        assay,
        evidence,
        hop,
    })
}

/// Parse one node scroll. `coord` is the scroll's coordinate in its home
/// phext. On failure, returns every violation found in the scroll.
pub fn parse_node(coord: &str, text: &str) -> Result<Node, Vec<String>> {
    let mut errors: Vec<String> = Vec::new();
    let mut node: Option<Node> = None;
    let mut assay_lines = 0usize;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if node.is_none() {
            // The first content line of a node scroll is its node line.
            let mut parts = line.splitn(3, " · ");
            match (parts.next(), parts.next(), parts.next()) {
                (Some("node"), Some(kind), Some(statement)) => {
                    if kind == "person" {
                        errors.push(
                            "kind `person` is non-conformant: KGOAT is for ideas, not people (D6, SPEC §5.2)"
                                .to_string(),
                        );
                    } else if !schema::is_node_kind(kind) {
                        errors.push(format!("unknown node kind `{kind}` (SPEC §1.2)"));
                    }
                    node = Some(Node {
                        coord: coord.to_string(),
                        kind: kind.to_string(),
                        statement: statement.trim().to_string(),
                        provenance: None,
                        assay: None,
                        edges: Vec::new(),
                        history: Vec::new(),
                    });
                }
                _ => {
                    errors.push(format!(
                        "scroll does not open with a `node · <kind> · <statement>` line: `{line}`"
                    ));
                    return Err(errors);
                }
            }
            continue;
        }
        let n = node.as_mut().expect("node initialized above");
        let tokens: Vec<&str> = line.split(" · ").collect();
        match tokens[0] {
            "edge" => match parse_edge(&tokens, line) {
                Ok(e) => n.edges.push(e),
                Err(e) => errors.push(e),
            },
            "assay" => {
                assay_lines += 1;
                if tokens.len() != 4 {
                    errors.push(format!("assay line has {} fields, want 4: `{line}`", tokens.len()));
                    continue;
                }
                match parse_assay_pair(tokens[1]) {
                    Ok((status, basis)) => {
                        if tokens[2].is_empty() || tokens[2] == "-" {
                            errors.push("claim node assay has no evidence pointer (SPEC §5.1)".to_string());
                        }
                        if !schema::hop_stamp_well_formed(tokens[3]) {
                            errors.push(format!("assay hop stamp `{}` is not an ISO date", tokens[3]));
                        }
                        n.assay = Some(Assay {
                            status,
                            basis,
                            evidence: tokens[2].to_string(),
                            hop: tokens[3].to_string(),
                        });
                    }
                    Err(e) => errors.push(e),
                }
            }
            "provenance" => {
                let mut parts = line.splitn(2, " · ");
                let _ = parts.next();
                if let Some(text) = parts.next() {
                    n.provenance = Some(text.trim().to_string());
                }
            }
            "history" => {
                if tokens.len() != 4 {
                    errors.push(format!(
                        "history line has {} fields, want 4: `{line}`",
                        tokens.len()
                    ));
                    continue;
                }
                let hop = tokens[1];
                if !schema::hop_stamp_well_formed(hop) {
                    errors.push(format!("history hop stamp `{hop}` is not an ISO date"));
                    continue;
                }
                let parsed = tokens[2].split_once("->").and_then(|(f, t)| {
                    match (Status::parse(f), Status::parse(t)) {
                        (Some(from), Some(to)) => Some((from, to)),
                        _ => None,
                    }
                });
                match parsed {
                    Some((from, to)) => {
                        if !schema::transition_legal(from, to) {
                            errors.push(format!(
                                "status transition {}->{} is not a legal MVO transition (SPEC §2.4)",
                                from.name(),
                                to.name()
                            ));
                        }
                        n.history.push(Transition {
                            hop: hop.to_string(),
                            from,
                            to,
                            evidence: tokens[3].to_string(),
                        });
                    }
                    None => errors.push(format!("history line has an unparseable transition: `{line}`")),
                }
            }
            _ => {
                // Body text: part of the page, not part of the stored graph form.
            }
        }
    }

    let n = match node {
        Some(n) => n,
        None => {
            errors.push("empty scroll: no node line".to_string());
            return Err(errors);
        }
    };
    // Exactly one assay record on claim-bearing node units, none elsewhere:
    // the registry stays total and unique (SPEC §5.1).
    if n.kind == "claim" && assay_lines != 1 {
        errors.push(format!(
            "claim node carries {assay_lines} assay lines, want exactly 1 (SPEC §5.1)"
        ));
    }
    if n.kind != "claim" && assay_lines > 0 {
        errors.push("non-claim node carries an assay line; node-level assay belongs to claim nodes (SPEC §5.1)".to_string());
    }
    if errors.is_empty() {
        Ok(n)
    } else {
        Err(errors)
    }
}
