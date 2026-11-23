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

## Remaining legs of this stage

- The registers: artifacts.phext (works), skills.phext (works),
  tasks.phext (works in flight), state.phext (events and metrics) —
  same treatment, counts reconciled against each register's own
  records.
- Cross-corpus edges: the first stored edges between homes —
  research → ietf `mentions` only where a scroll names an RFC as an
  RFC (curated, verified by reading), and the Research Oracle
  metric —measures→ the works it prices.
