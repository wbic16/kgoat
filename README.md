# KGOAT — Knowledge Graph Of All Things

> "KGOAT organizes everything you enter holographically."

> "KGOAT doesn't require you to understand the tech, it just works."

knowledge graph of all things, a layer 2 phext format

KGOAT is comprised of three major components:

- **worker ⇒ phext-lattice** — where scrolls are made, edited, and live
- **warrior ⇒ tluf** — the assay layer: claims and associations tested against evidence
- **religious ⇒ scrollspace** — the traversal surface: paths walked through scroll space

KGOAT is for ideas, not people. Its subject matter is works, concepts,
claims, artifacts, and their relations — there are no person nodes, and
authorship rides as attribution on a work's scroll, never as a node.

## This repository

- **SPEC.md** — the format specification: the node/edge grammar, the
  federated storage model, the assay discipline, and the build sequence.
- **src/schema.rs** — the format schema as machine-checkable definitions:
  the kind table, the edge catalogue, the basis→status legality table, and
  the legal status transitions.
- **src/scroll.rs** — the node-scroll parser: reads a scroll into its
  kind, statement, assay record, outbound edges, and history, and reports
  every format violation it finds.
- **src/builder.rs** — the index builder: folds the node scrolls of a
  federation of phexts into the derived surfaces — the inbound-edge index,
  the kind/type tables, and the assay registry — rendered as a phext.
  The scrolls are right; the index is a calculation over them.
- **src/main.rs** — `kgoat-build`, the builder CLI:

```text
kgoat-build <home>=<file.phext> [<home>=<file.phext> ...] --out <index.phext>
```

Exit status is 1 when the graph carries schema violations.

This is the initial tooling (SPEC §6, Stage 0's machinery): schema,
parser, and builder, with tests. It does not yet ingest the seed corpora,
run the traversal service, or serve the phext.tech surface — those are
Stages 1–4 of the spec's build sequence.

Built on libphext, the reference implementation of phext. MIT licensed.
