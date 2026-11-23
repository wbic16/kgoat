# KGOAT Stage 2 — corpus graph layers (SPEC §6 Stage 2, §4.1)

Branch: emise/kgoat-stage1 · design: DESIGN.md Addendum 5 ·
built on aletheia-core under Will's go-ahead of 2026-11-23
("finish kgoat first", then "Go ahead — work in the kgoat clone").

## Research leg — LANDED (EE ~348, 2026-11-23)

`src/bin/kgoat-research-gen.rs`: reads research.phext and the MOAT
registry (registry.json, emitted by tluf's own extractor,
research-tracker/epistemic/) and writes a working copy carrying the
graph layer in volume 1.3.1. Production research.phext untouched.

The gate, before anything is written: the registry's recorded
SHA-256 must equal the phext's actual SHA-256 (418d0676…), and the
registry must be total over the phext's scrolls in both directions.
The assignments are inherited verbatim (§5.1) — KGOAT re-judges
nothing.

Per registry unit, the ietf pattern: a content node at
1.1.1/1.3.1/1.2.<registry id> (kind by the extractor's coordinate
classes: concepts/foundations/areas → concept; meta and registry
pages → work; the Research Oracle ledger 1.1.4 → metric; paper and
study chapters → work) and a claim node at 1.1.1/1.3.1/1.4.<id>
carrying the unit's MOAT assay. Every generated node part_of the
research work node at 1.1.1/1.3.1/1.1.1.

The one evidence transformation, named in the graph Home: where
the registry records no pointer ("—", the stipulations), the assay
evidence is the unit's own coordinate. A " · " inside an evidence
string would be written " — " (the assay line's separator); the
current registry needed neither this nor any line-collapsing —
0 fields adjusted.

Result (hub /home/wbic16/kgoat-stage1/out/):
- research-graph.phext — 3,563,836 B, sha256 6416572d7291e25dd122
  a86e6b8b98a3cfc4dca5e6205b81354b1d227b3d1bb2 · fidelity PASS
  (every original scroll byte-identical).
- Fold of the research home: 2,543 nodes · 2,542 edges · 1,271
  registry entries · 0 violations. Kinds: concept 857 · work 413 ·
  metric 1. Statuses, matching the MOAT registry exactly:
  True 513 · Unknown 749 · False 2 · Fiction 7.
- Federation fold (ietf + research): 22,736 nodes · 127,334 edges ·
  11,323 registry entries · 0 violations. No cross-corpus edges
  exist yet — a path query between the homes correctly finds none;
  that is the cross-corpus leg's work, below.

Tests: 21/21 green (7 stage-0 + 6 ietf generator + 5 traversal +
3 research generator, including the sha/totality gates and a fold
of a fixture layer through the builder).

## Registers leg — LANDED (EE ~357, 2026-11-23)

`src/bin/kgoat-register-gen.rs` (DESIGN.md Addendum 6): one
generator, four modes, writing artifacts-graph.phext,
skills-graph.phext, tasks-graph.phext, state-graph.phext — working
copies carrying the graph layer in volume 1.3.1, production
registers untouched. Per register: a work node (graph Home) at
1.1.1/1.3.1/1.1.1 and one content node per unit at
1.1.1/1.3.1/1.2.<ordinal> (ordinal = the register's own order),
every content node part_of the work node at w0 with the unit's
source coordinate as evidence. No claim nodes: none of the four
registers carries a MOAT assay layer, and no assay is fabricated
for a work or an event. A task's workflow status rides as a
`status-record` provenance line, never as an assay.

Gates, all passed before anything was written:

- **artifacts** (input snapshot 1,398,012 B, sha256 33fd0bc3…,
  2026-11-19): artifacts-source.json's 59 entries == the phext's
  60 non-empty scrolls less Home; every entry's scroll carries a
  provenance header whose SHA-256 and Title equal the source
  record exactly (59/59 receipts verified). Nodes carry the
  inscribed date, the sha256 receipt, and the source path.
- **skills** (1,286,729 B, sha256 f99e7b5a…): the phext stores one
  scroll per file (249 files); the Home index's entries == the
  non-Home scroll set exactly, and each entry's printed sha256 is
  a prefix of the sha256 computed over its scroll (249/249). The
  unit is the skill: 170 skills grouped by the index, each a work
  node whose body lists every file with its full computed sha256.
  Persona skills are capabilities like any other — the node is
  the skill work, never a person (§5.2).
- **tasks** (3,710,872 B, sha256 e8a1e7df…): tasks-source.json's
  857 records == 858 non-empty scrolls less Home; every record
  coordinate present.
- **state** (1,056,891 B, sha256 027c7eaa…, 2026-11-21): the phext
  is its own source of truth — 3,760 distilled ranch states, each
  an event node; every non-empty line of every unit carries the
  " | " entry format (0 violations); 4,447 occurrences in the
  snapshot. A node's statement is its first line's summary;
  numbers stay inside the statement text, as the register's own
  design carries them.

Folds (hub /home/wbic16/kgoat-stage1/out/, all 0 violations):
artifacts 60 nodes · 59 edges · artifacts-graph.phext 1,420,338 B
sha256 b1727ca5… · skills 171 nodes · 170 edges ·
skills-graph.phext 1,361,493 B sha256 c074a83f… · tasks 858 nodes
· 857 edges · tasks-graph.phext 4,087,100 B sha256 53ce10e9… ·
state 3,761 nodes · 3,760 edges · state-graph.phext 2,646,996 B
sha256 0e4075a4…. **Federation fold, six homes: 27,586 nodes ·
132,180 edges · 11,323 registry entries · 0 violations**
(federation-index.phext sha256 f2b8392e…). Tests: 26/26 green
(7 stage-0 + 6 ietf gen + 5 trav + 3 research gen + 5 register
gen), zero warnings.

## Remaining leg of this stage

- Cross-corpus edges: the first stored edges between homes —
  research → ietf `mentions` only where a scroll names an RFC as an
  RFC (curated, verified by reading), and the Research Oracle
  metric —measures→ the works it prices.
