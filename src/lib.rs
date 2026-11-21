//! KGOAT — Knowledge Graph Of All Things.
//!
//! A layer 2 phext format (see SPEC.md). This crate is the initial tooling:
//! the format schema as machine-checkable definitions ([`schema`]), a parser
//! for node scrolls ([`scroll`]), and the builder that folds node scrolls
//! into the derived surfaces — inbound-edge index, kind/type tables, and the
//! assay registry ([`builder`]). The scrolls are the storage; everything the
//! builder emits is a calculation over them (SPEC §2.3).

pub mod builder;
pub mod schema;
pub mod scroll;
