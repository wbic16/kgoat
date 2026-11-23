# KGOAT Stage 1 — the ietf slice (SPEC §6)

Status: COMPLETE, 2026-11-23 (EE ~344). Built on branch
`emise/kgoat-stage1` over the stage-0 freeze (caa81e6).

## What was built

`src/bin/kgoat-ietf-gen.rs` — a generator that reads the ietf corpus
(ietf.phext + ietf-source.json) and writes a working copy,
ietf-graph.phext, carrying the graph layer in book 1.3.1 of the home
phext, beside the corpus material, per SPEC §2.5:

- corpus work node — 1.1.1/1.3.1/1.1.1
- document nodes — 1.1.1/1.3.1/1.2.<rfc> (10,052)
- project nodes — 1.1.1/1.3.1/1.3.<n>, alphabetical (88)
- record-claim nodes — 1.1.1/1.3.1/1.4.<rfc> (10,052), each carrying
  the RFC's MOAT assay (status/basis from the record, evidence = the
  document scroll coordinate) — assay lines are legal only on claim
  nodes, so the record claim is modeled as one.

Edges stored once, in the §2.2 grammar, all w0 (the source stores no
weights; §1.3 forbids defaulting to 3): references in three section
groups (normative / informative / kind-unspecified), supersedes from
`obsoletes` (the reverse is derived by the index, never stored),
implements on the project node with per-link role/basis/evidence as
record lines, part_of to the corpus work node. Structural edges omit
the status/basis pair. Hop stamp 2026-11-23.

The stage-0 builder then folds the working copy into the generated
index phext (§2.3): inbound-edge index, kind/type tables, assay
registry, Home roll-up — counts calculated from the scrolls.

## Sources and the cross-check

ietf-source.json is the corpus's declared source of truth (ietf.phext
Home). Complete edge lists are read from it, because the document
scrolls truncate long reference lists at 60 printed entries behind a
`(+N more in ietf-source.json)` marker (see below). The generator
cross-checks the two readings before writing anything: every
scroll-printed list must be an exact prefix of the JSON list, with
any shortfall accounted for exactly by the scroll's own truncation
marker; implementation names must match per RFC. Any other
disagreement stops generation with the divergences printed — nothing
is written, nothing is silently absorbed.

## Results — every audited count reproduced exactly

| count | audited | built |
|---|---|---|
| records / document nodes | 10,052 | 10,052 |
| reference edges | 96,168 | 96,168 |
| implementation links | 6,888 | 6,888 |
| projects | 88 | 88 |
| superseded RFCs | 1,385 | 1,385 |
| supersedes edges | 1,544 | 1,544 |
| updates pairs (prose, see gap) | 2,169 | 2,169 |

Builder fold: 20,193 nodes (document 10,052 · claim 10,052 · project
88 · work 1) · 124,792 edges (references 96,168 · part_of 20,192 ·
implements 6,888 · supersedes 1,544) · 10,052 assay-registry entries ·
0 violations · 0 edges to non-node targets. Fidelity: every original
scroll of ietf.phext is byte-identical in the working copy
(explode→implode proof inside the generator). Tests: 13/13 green
(7 stage-0 + 6 new).

Artifacts (on aletheia-core, /home/wbic16/kgoat-stage1/out/):
- ietf-graph.phext — 584,249,378 B — sha256
  346213441b9ce647aa014a628fc0df53f0a5b1259aafc2df8bc1525ce282fb69
- ietf-index.phext — 11,079,709 B — sha256
  48fc4bc619db43a3451092a65c6596eca4b397b3b51e1bc24316c3290ec7a97b

Production ietf.phext was never modified; the graph layer lives in
the working copy.

## Recovery / Invention conformance (§5.4)

- Recovery: the dense families (references, implements, supersedes)
  reproduce with direction, type, and counts exact against the
  audited corpus; the reverse directions exist only in the derived
  inbound index.
- Invention: person nodes 0 · kinds or edge types outside the §1.2 /
  §1.3 tables 0 (builder violations 0) · stored reverse edges 0 ·
  stored weights above w0: 0 · private material: none in the corpus.
- Named gap: `updates`/`updated_by` (2,169 pairs) are NOT drawn —
  the frozen v1 edge catalogue has no `updates` type. They are
  recorded as provenance lines on the document nodes and named as a
  catalogue gap in the graph Home. Drawing them needs a catalogue
  decision, not an invention.

## Finding that bears on rfcbox

The ietf.phext document scrolls do not store the complete reference
graph: lists longer than 60 entries print their first 60 numbers
plus a `(+N more in ietf-source.json) (M edges)` marker. 21 RFCs
carry truncated lists; 1,042 references live only in
ietf-source.json. Reference kinds split normative 44,214 ·
informative 39,331 · kind-unspecified 12,623. Any consumer reading
edges from the scrolls alone will undercount; the JSON is the
complete store, as the corpus Home declares.

## Next

Stage 1's surface leg (traversal service §3.1 + node pages), then
Stages 2–4 per SPEC §6.
