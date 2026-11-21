//! The KGOAT format schema as machine-checkable definitions (SPEC §§1.2,
//! 1.3, 2.2, 2.4, 5.1, 5.2). Everything in this module is a table or a
//! predicate over tables: the parser and the builder enforce the format by
//! consulting these definitions, never by re-stating them.

/// Node kinds (SPEC §1.2). Eight kinds, each supplied by a seed corpus.
/// There is deliberately no `person` kind: KGOAT is for ideas, not people
/// (D6, SPEC §5.2). A scroll claiming kind `person` is non-conformant.
pub const NODE_KINDS: [&str; 8] = [
    "org", "work", "concept", "claim", "document", "event", "metric", "project",
];

/// Is `kind` a v1 node kind?
pub fn is_node_kind(kind: &str) -> bool {
    NODE_KINDS.contains(&kind)
}

/// Edge types (SPEC §1.3). Every type below already exists in a seed corpus.
/// `made` is NOT an edge type: it is attribution recorded on a work's scroll
/// (SPEC §1.2), because there is no person node for the edge to leave from.
pub const EDGE_TYPES: [&str; 9] = [
    "references",
    "implements",
    "supersedes",
    "derived_from",
    "asserts",
    "part_of",
    "contradicts",
    "measures",
    "mentions",
];

/// Is `ty` a v1 edge type?
pub fn is_edge_type(ty: &str) -> bool {
    EDGE_TYPES.contains(&ty)
}

/// Edge types whose existence asserts something about the world (SPEC §5.1).
/// A claim-bearing edge MUST carry exactly one status/basis pair and a real
/// evidence pointer. `derived_from` MAY carry the pair (a claim's provenance
/// is assayable); all other types MUST NOT carry it — a structural edge
/// omits the pair rather than decorating itself with an assay it does not
/// need (SPEC §2.2).
pub const CLAIM_BEARING_REQUIRED: [&str; 2] = ["asserts", "contradicts"];
pub const CLAIM_BEARING_ALLOWED: [&str; 1] = ["derived_from"];

/// Does an edge of type `ty` require a status/basis pair?
pub fn assay_required(ty: &str) -> bool {
    CLAIM_BEARING_REQUIRED.contains(&ty)
}

/// May an edge of type `ty` carry a status/basis pair at all?
pub fn assay_permitted(ty: &str) -> bool {
    assay_required(ty) || CLAIM_BEARING_ALLOWED.contains(&ty)
}

/// Maximum weight for an edge type. Weights are ordinal 0–3 (SPEC §1.3);
/// `mentions` — the weakest form — is capped at 1.
pub fn max_weight(ty: &str) -> u8 {
    if ty == "mentions" {
        1
    } else {
        3
    }
}

/// Claim statuses (SPEC §5.1), inherited from the Minimum Viable Oracle
/// discipline whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    True,
    False,
    Unknown,
    Fiction,
}

impl Status {
    pub fn parse(s: &str) -> Option<Status> {
        match s {
            "True" => Some(Status::True),
            "False" => Some(Status::False),
            "Unknown" => Some(Status::Unknown),
            "Fiction" => Some(Status::Fiction),
            _ => None,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Status::True => "True",
            Status::False => "False",
            Status::Unknown => "Unknown",
            Status::Fiction => "Fiction",
        }
    }
}

/// Claim bases (SPEC §5.1). The basis constrains which statuses a
/// claim-bearing unit may take — see [`status_legal_for_basis`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Basis {
    Stipulation,
    Verified,
    Refuted,
    Attributed,
    Open,
    Synthesis,
    Estimate,
    Construction,
}

impl Basis {
    pub fn parse(s: &str) -> Option<Basis> {
        match s {
            "STIPULATION" => Some(Basis::Stipulation),
            "VERIFIED" => Some(Basis::Verified),
            "REFUTED" => Some(Basis::Refuted),
            "ATTRIBUTED" => Some(Basis::Attributed),
            "OPEN" => Some(Basis::Open),
            "SYNTHESIS" => Some(Basis::Synthesis),
            "ESTIMATE" => Some(Basis::Estimate),
            "CONSTRUCTION" => Some(Basis::Construction),
            _ => None,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Basis::Stipulation => "STIPULATION",
            Basis::Verified => "VERIFIED",
            Basis::Refuted => "REFUTED",
            Basis::Attributed => "ATTRIBUTED",
            Basis::Open => "OPEN",
            Basis::Synthesis => "SYNTHESIS",
            Basis::Estimate => "ESTIMATE",
            Basis::Construction => "CONSTRUCTION",
        }
    }
}

/// The basis→status legality table (SPEC §5.1, MVO discipline): no VERIFIED
/// receipt, no True; a refuted claim is False/REFUTED; attributed and open
/// material is believed by no one yet (Unknown); a construction is Fiction.
pub fn status_legal_for_basis(status: Status, basis: Basis) -> bool {
    use Basis::*;
    use Status::*;
    match basis {
        Stipulation => status == True,
        Verified => status == True,
        Refuted => status == False,
        Attributed => status == Unknown,
        Open => status == Unknown,
        Synthesis => status == Unknown,
        Estimate => status == Unknown,
        Construction => status == Fiction,
    }
}

/// Legal status transitions (SPEC §2.4, the MVO legal edges). Every
/// transition is recorded as a history event with its evidence pointer.
/// There is no transition into True without a receipt, and Fiction never
/// goes directly to True.
pub fn transition_legal(from: Status, to: Status) -> bool {
    use Status::*;
    matches!(
        (from, to),
        (Unknown, True)
            | (Unknown, False)
            | (True, False)
            | (False, Unknown)
            | (Fiction, Unknown)
            | (Unknown, Fiction)
    )
}

/// Is `s` a well-formed phext coordinate string (`a.b.c/d.e.f/g.h.i`, each
/// component ≥ 1)? Coordinates are content, not keys (SPEC §2.1); this
/// checks form only.
pub fn coordinate_well_formed(s: &str) -> bool {
    let groups: Vec<&str> = s.split('/').collect();
    if groups.len() != 3 {
        return false;
    }
    groups.iter().all(|g| {
        let parts: Vec<&str> = g.split('.').collect();
        parts.len() == 3
            && parts
                .iter()
                .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
            && parts.iter().all(|p| p.parse::<usize>().map(|n| n >= 1).unwrap_or(false))
    })
}

/// Is `s` a well-formed hop stamp (SPEC §2.2): an ISO date `YYYY-MM-DD`,
/// optionally followed by `T` and a time. The graph records when *it*
/// learned a thing; hop stamps are the graph's own history (SPEC §5.2).
pub fn hop_stamp_well_formed(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() < 10 {
        return false;
    }
    let digit = |i: usize| b[i].is_ascii_digit();
    digit(0)
        && digit(1)
        && digit(2)
        && digit(3)
        && b[4] == b'-'
        && digit(5)
        && digit(6)
        && b[7] == b'-'
        && digit(8)
        && digit(9)
        && (b.len() == 10 || b[10] == b'T')
}
