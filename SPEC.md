# KGOAT — Knowledge Graph Of All Things

**Design spec** · 2026-11-21 · task 24.8.9/11.9.12/10.8.11
**Revised 2026-11-21** — component ruling (Will, main chat, 15:43 CST, verbatim):
"phext.tech is now pointed at awsbox, revise kgoat to be comprised of three
major components { worker => phext-lattice, warrior => tluf, religious =>
scrollspace }". The three components are defined in §0.1; §7A carries the
decision accounting.
**Revised again 2026-11-21** — decisions D1–D6 ruled by Will (main chat,
15:46–15:47 CST) and applied throughout. §7 is frozen as RULED; §7A is
the decision accounting of record.
**Status: DESIGN ONLY.** No build is authorized by this document. Nothing here is published.
**Build home of record:** wbic16/kgoat — https://github.com/wbic16/kgoat
(public, default branch `exo`, LICENSE only at spec time).
**Will's repo description, verbatim:** "knowledge graph of all things, a layer 2 phext format."

KGOAT is the layer-two composition at the heart of the phext.tech launch.
Its three major components are existing systems (§0.1): worker ⇒
phext-lattice, warrior ⇒ tluf, religious ⇒ scrollspace. phext.tech was
registered by Will via NFSN on 2026-11-21 and pointed at awsbox the same
day (verified from aletheia-core: A → 44.248.235.76, awsbox's public
ingress; vhosts and TLS still to come).

## 0. Name, definition, axioms

**Name: KGOAT** (Knowledge Graph Of All Things) — named by Will, 2026-11-21.
The naming question is closed; there is no open naming decision in this spec.

**Definition, Will's words, verbatim:** "KGOAT organizes everything you enter
holographically."

Two axioms govern everything below. Where a design choice conflicts with
either, the axiom wins and the choice is wrong.

- **Axiom 1 — organization is holographic.** Material is addressed by a
  holographic coordinate computed from its own content — the same discipline
  as the task coordinates and the artifacts.phext inscription, where a
  coordinate is computed from the artifact's own words via the Singular merge
  (tokenize as the Singular checker does; take each distinct word's
  holographic position from the Singular atlas; average component-wise;
  round half up, never below 1). Nothing is filed into a hand-maintained
  taxonomy. There is no tree to curate because there is no tree: placement is
  a function of content, and retrieval is navigation of the space the
  content itself spans.
- **Axiom 2 — the tech is invisible.** Will, verbatim: "KGOAT doesn't require
  you to understand the tech, it just works." A user needs no phext
  knowledge, never sees a coordinate unless they go looking for it, and makes
  no taxonomy decisions. They enter things in plain form; the organization
  happens automatically. Coordinates, statuses, and edge types are
  discoverable depth — present for anyone who digs, never a prerequisite
  for anyone who doesn't.

**What KGOAT is, in three layers** (aligned with Will's framing of KGOAT as
a layer-2 phext *format*):

1. **The format** — a node/edge grammar expressed in phext: how a scroll
   declares what it is, what it connects to, and how strongly, on top of the
   layer-one substrate (phext, libphext, SQ, phext-lattice). This is the
   durable artifact; it is what wbic16/kgoat primarily holds.
2. **The graph** — the first instance of the format: a graph seeded from
   corpora that already exist as phexts (see §5), federated across those
   phexts from the start (D4, §2.5) — node scrolls live in their home
   phexts, and the graph is the federation, addressed by (home phext,
   coordinate).
3. **The surface** — the phext.tech web presence: search, read, walk, and
   (for logged-in users) enter things. Built in the Phext, Inc. design
   language (§3.3). In the component terms of §0.1, the surface is the
   religious component (scrollspace) presenting the graph, and entry rides
   the worker component (phext-lattice).

Layer two means exactly this: KGOAT assumes layer one exists and adds
*typed, assayed connection* to it. Layer one stores text with addresses.
Layer two says what the addressed things are and how they relate — and
carries the receipts for saying so.

### 0.1 The three components (Will's ruling, 2026-11-21 15:43 CST)

KGOAT is comprised of three major components — **worker ⇒ phext-lattice,
warrior ⇒ tluf, religious ⇒ scrollspace**. These are not pages, views, or
themes, and they are not new builds: each is an existing, shipped system,
and KGOAT is their composition. This supersedes both earlier framings in
this spec's lineage (the 15:28 page-mapping and the 15:30 abstract-phase
cycle). What each component is, grounded in what the named system actually
is:

- **Worker ⇒ phext-lattice.** phext-lattice is the shipped editor stack:
  the web layer the platform runs on, serving Phext Edit at edit.phext.io
  (phext-lattice v1.3.1 on crates.io; editor launched on Product Hunt
  2026-11-10). The worker component is where scrolls are made, edited, and
  live. In KGOAT, node scrolls are ordinary scrolls to the worker: entry
  (§4.4), node-page authoring, and the graph's editing surface all ride
  the stack that already exists. *What the composition still needs from
  it:* nothing structural — the format's scrolls (§2) are content the
  worker already hosts, in the home phexts where they live (§2.5); the
  join is that the graph's node scrolls are among its phexts.
- **Warrior ⇒ tluf.** TLUF (tluf.org) is the assay layer: its machinery
  (wbic16/moat) is what runs the Minimum Viable Oracle discipline — every
  claim carrying exactly one status, a basis, and an evidence pointer —
  over research.phext today. The warrior component tests claims and
  associations against evidence. In KGOAT, §5 is the warrior's section:
  the assay registry over the graph's claim-bearing units, derivation
  grading, and the Recovery/Invention conformance runs are tluf's work,
  extended from research.phext to the federated graph. *What the composition still
  needs from it:* the registry generated over the graph (the builder's
  registry of §2.3/§5.1, produced by tluf's discipline), not a new
  epistemics.
- **Religious ⇒ scrollspace.** Scrollspace is the traversal surface: the
  hosted corpus, viewable and walkable without a login (the Scrollspace
  rule — a login is required only to edit). The religious component walks
  paths through scroll space. In KGOAT, §3.2's visitor surface — search,
  node pages, walking connection to connection — is scrollspace
  presenting the graph. *What the composition still needs from it:*
  scrollspace extended from walking corpus scrolls to walking the graph:
  node pages and path traversal across the graph's home phexts, on the
  same no-login terms.

**The composition itself is the only genuinely new thing.** None of the
three components alone is KGOAT. What does not exist yet is the join: the
graph layer (§2) — node scrolls in their home phexts, which the worker
hosts, whose claims the warrior assays, and whose paths the religious
walks — plus the traversal service (§3.1) and the phext.tech surface
presenting all three as one product. The build sequence (§6) is written in exactly those terms: what
already exists, and what the composition still needs.

## 1. What the graph is

### 1.1 The honest scope of "all things"

"Of all things" is the direction, not the v1 scope. The seam, stated
plainly:

- **v1's "all" = everything that has entered through a phext door** — our
  own corpus: the registers (tasks, artifacts, skills, state), research.phext,
  the agi corpus, the people/group pages, and ietf.phext. At the last full
  count (2026-11-16 audit): ~13,800 register scrolls (tasks 545 · agi 10,140
  · state 2,090 · research 822 · artifacts ~50 · skills 124) plus ietf.phext's
  10,052 records. That is a graph of roughly 24,000 nodes and — counting
  ietf.phext's stored edges alone — over 103,000 edges, before a single new
  thing is entered. One seed's status is stated here so it is never
  blurred later: ietf.phext is a *finished* artifact — built 2026-11-18,
  three-leg audited to a final READY verdict, hosted and hash-verified —
  and KGOAT generalizes it as completed prior art (§4.1). The work in
  progress is KGOAT, not its seeds.
- **Launch scope is settled (D2, §7).** phext.tech launches with access
  to **all phexts currently being curated** — the whole seed set is
  launch material, not a first slice of it. The ietf slice is the deepest
  pre-extracted component of the launch, not its boundary (§6).
- **The universal claim belongs to the format, not the seed.** The format
  places no limit on what can be entered; the v1 graph is one operator's
  corpus, honestly labelled as the seed. Any public surface that says "all
  things" must also say, in the same breath, what has actually been entered
  so far — the counts are the receipts, per the design language's
  evidence-first copy rule.

### 1.2 Nodes are scrolls

A KGOAT node is not a row in a database. **A node is a scroll**: a page of
plain text at a coordinate in a phext. Node scrolls live in their home
phexts (§2.5), beside the underlying material; a node's scroll is its
page in the graph — what it is, what kind it is, its outbound edges, its
assay record, its history. This follows the standing rules directly: a scroll is
a page, not a cell (lines and columns inside it are free — use them first),
and phext outputs are self-contained (no sidecars; anything a node depends
on is kept inside the chain).

Every node carries a **kind**, from a fixed table. The table is small on
purpose: a kind exists in v1 only if a seed corpus already supplies nodes
of that kind.

| Kind | What it is | Supplied by |
|---|---|---|
| org | an organization, company, community | group pages; ietf.phext projects' homes |
| work | a made thing: repo, site, paper-as-built, product, skill, task | artifacts.phext, skills.phext, tasks.phext, GitHub walk |
| concept | a canonical idea, one scroll per concept | research.phext |
| claim | a single assertion that can be assayed | MOAT / research.phext registry |
| document | a source text held whole | ietf.phext (RFCs), incipit/agi texts |
| event | something that happened at a time: a launch, a release, a decision | state.phext, launch archive |
| metric | a measurement: value, as-of, confidence | bench results (holo-bench), corpus counts, accounting |
| project | a living engineering effort with a repo and a state | ietf.phext's 88 implementation projects; rfcbox programme |

Eight kinds — one fewer than Boardy's table, and the difference is the
point: Boardy's graph is of a person's network; KGOAT has no person kind
at all (D6, §5.2). The kinds that remain are re-derived from our corpora
rather than copied (see §1.4).

A kind is an attribute of the scroll, recorded on its first line block —
never a separate registry row. A node whose kind a source does not supply
does not get one inferred silently: it enters as the most general applicable
kind its evidence supports, and the gap is visible on its page.

**People appear in KGOAT only as attribution, never as nodes.** A work's
scroll records who made it — a name in its provenance lines, an evidence
pointer to where that is attested — because provenance is part of a
work's receipts. That is a statement *about the work*, stored on the
work. It does not make the person a node: there is no person scroll, no
person page, nothing to walk to. Authorship is evidence; people are not
subject matter (D6, §5.2).

### 1.3 Edges are lines

An edge is a typed, directed, weighted statement stored **once**, as lines
inside the source node's scroll (grammar in §2.2). Direction is part of the
claim: an edge and its reverse are different statements. Inbound edges are
never stored — they are derived by the index (§2.3), so every edge has
exactly one stored form and one author of record.

Weights follow Boardy's discipline, adopted whole: an ordinal 0–3 judgment
of how strong the link actually is (3 load-bearing, 2 real but secondary,
1 weak or indirect, 0 recorded with no strength). Weights are copied where
stored, never defaulted to 3; uniform 3s are inflation and a defect.

The v1 edge catalogue — every type below already exists in a seed corpus;
no type is invented for symmetry:

| Type | Direction | Seed evidence |
|---|---|---|
| references | document → document | ietf.phext: 96,168 stored reference edges |
| implements | project → document | ietf.phext: 6,888 implementation links, 88 projects |
| supersedes | document → document | ietf.phext: 1,385 superseded RFCs, annotated |
| derived_from | claim/concept → document/work | research.phext: each paper stored as its delta against canonical concept scrolls |
| asserts | org → claim | MOAT registry: claims carry their asserters; a person asserter rides as attribution on the claim scroll (§1.2), never as a node |
| made | *(attribution, not an edge)* | artifacts register; the GitHub walk's authorship record — recorded on the work's scroll as provenance (§1.2); there is no person node for the edge to leave from |
| part_of | any → work/concept | register structure: scrolls belong to collections |
| contradicts | claim → claim | MOAT refutations; research.phext False entries |
| measures | metric → work/org/concept | bench receipts; corpus counts |
| mentions | any → any | the corpus's implicit edge, made explicit — weight capped at 1, always labelled as the weakest form |

### 1.4 What carries over from Boardy's work — and what doesn't

Boardy's spec and boot manual (2026-11-18) are the closest prior art inside
this programme: a graph format tested by whether a fresh boot can recover
what is present and respect what is deliberately absent. Carries over:

- **Typed, directed, weighted edges with fixed kind attribute sets.** The
  discipline, not the specific table (see below).
- **Weights as ordinal judgments** with a tolerance of one step between
  honest readers, never two (§1.3).
- **The dense/empty doctrine.** Some edge families carry nearly everything;
  some are empty *on purpose*, and their emptiness is content. KGOAT adopts
  this as schema law (§5.2).
- **Derivation grading.** An edge or answer is graded on its derivation —
  the shown steps from sources — not on its landing spot (§5.3). This was
  Boardy's amendment to the accelerator test; it is KGOAT's rule for every
  derived edge.
- **Recovery / Invention as the conformance test** for any boot or import
  (§5.4).

Does not carry over:

- **Boardy's specific kinds ask, blocker, channel.** Those are the
  semantics of one person's introduction network — his instance, not the
  general graph. They are not in the v1 table. If a domain needs them, they
  arrive later as a domain pack layered on the format, the way his instance
  itself would join (§4.3) — as a *source*, with its own kinds mapped at the
  border, not by growing the core table.
- **His five dense families as KGOAT's dense families.** KGOAT's dense
  families are its own, read off the seed corpora: document→references,
  project→implements, claim→derived_from, any→part_of. The doctrine
  transfers; the families are re-derived.
- **The person graph itself.** Boardy's instance is a graph *of* a
  person's network — people are its subject matter, and its empties
  protect them. KGOAT is for ideas, not people (D6): there are no person
  nodes for empties to protect. What reads as KGOAT's empties (§5.2) is
  the same restraint re-grounded as scope.


## 2. Storage and addressing

### 2.1 Identity is a coordinate

A node's identity is its address: **(home phext, coordinate)** — the phext
it lives in, and its 9D coordinate inside that phext. Nothing else is an
identifier. There are no UUIDs in KGOAT, no surrogate keys, no
auto-increment anything. This is the emissary rule (§4.2) applied at home
first: identities assigned by a system die at that system's border;
coordinates survive, because a coordinate is content-addressed text, not a
database's opinion.

**Placement is holographic (Axiom 1).** When a thing is entered, its
initial coordinate is computed from its own words by the Singular merge —
the same computation that places a task in tasks.phext and an artifact in
artifacts.phext. Two consequences are designed for, not discovered later:

- **Collision is normal.** Two things with near-identical word-stock land
  near each other — that is the point of the space — and occasionally on
  the same coordinate. The register precedent governs: a coordinate, once
  issued to a node, is that node's forever; a colliding entrant is placed
  at the nearest free coordinate by a deterministic rule recorded on both
  scrolls. Proximity is information; the nudge is bookkeeping.
- **An address is not a hash.** The coordinate is computed from the words
  at entry. The thing then changes — it accrues edges, statuses, history —
  and its address **does not move**. Re-coordinating on every edit would
  break every edge that points at it. Stability beats purity: the address
  records where the thing entered the space; the space is the map of
  entries, and a map whose streets rename themselves is not a map.

### 2.2 The edge line grammar

Edges live as plain-text lines inside the source node's scroll, in a fixed
line grammar — readable in a plain text editor, parseable by libphext
tooling, no brackets, no tags. (JSON, YAML, and XML appear nowhere in
KGOAT's storage or exchange; where an outside system forces them at a
border, exactly that border uses exactly that much, per standing rule.)

One edge per line, fields in fixed order, separated by ` · `:

```
edge · <type> · <target coordinate> · w<0-3> · <status>/<basis> · <evidence pointer> · <hop stamp>
```

- The status/basis pair appears only on claim-bearing edges (§5.1); a
  purely structural edge (part_of, references) omits it rather than
  decorating itself with an assay it doesn't need.
- The evidence pointer is a coordinate or a URL — always something a
  reader can open.
- The hop stamp (§2.4) records when this edge entered the graph.

A node's scroll, read top to bottom, is: what it is (kind + plain-language
statement of the thing), its assay record if it bears claims, its outbound
edges, and its history. Every scroll reads as a meaningful unit in a plain
text editor — the standing test for any phext structure.

### 2.3 The index is generated, never maintained

Nobody hand-edits an index of this graph. Following the register pattern
(tasks.phext's Home + builder; accounting.phext's calculated rollups), a
**builder** reads every node scroll and generates:

- the **inbound-edge index** (for each node, who points at it, with types
  and weights) — the reverse traversal the stored form deliberately omits;
- the **kind/type tables** (counts by kind, by edge type, by status);
- the **assay registry** (§5.1), grouped by status, rendered for readers;
- the **Home scroll** of each home phext's graph layer, and a federated
  roll-up across them: what the graph holds, counted from the scrolls,
  with the builder's stamp and the home phexts' hashes.

Every rollup is calculated from the stored atoms by the builder, never
hand-written as a summary number that can drift from the log. If the index
and a scroll disagree, the scroll is right and the index is rebuilt.

### 2.4 Updates and hop-time: history is kept, not overwritten

Phext supports hop-time — encoding the evolution of ideas in the address
space's own terms — and KGOAT's update model is built on it:

- A node's scroll is **append-mostly**. New edges are new lines. A changed
  status is a new history line recording the transition, its receipt, and
  its hop stamp — the old status is struck through in place, never deleted
  (the MVO rule: a claim falls True→False and the record gets *more*
  accurate; retirement is relocation with a redirect, never erasure).
- **Status transitions are legal only along the MVO edges** (Unknown→True
  on a verification receipt, Unknown→False on a refutation receipt,
  True→False, False→Unknown on an invalidated receipt, Fiction→Unknown,
  Unknown→Fiction) and every transition is itself a recorded event with its
  evidence pointer. There is no transition into True without a receipt, and
  Fiction never goes directly to True.
- The graph's *current* state — what a query answers from — is the
  builder's fold over the history, exactly as a register's current totals
  are a fold over its session atoms. History is the storage; the present is
  a calculation.

### 2.5 Where it physically lives — a federation, from the start

Settled by D4 (Will, 2026-11-21): **"Federation of phexts from the start,
like edit.phext.io — phext.tech is a direct upgrade."** There is no
single graph.phext. The model is the one edit.phext.io already runs:
many phexts, each hosted, each its own surface, joined by reference.

- **Each curated phext carries its own graph layer.** Node scrolls live
  in the home phext of the material they describe — ietf.phext's nodes in
  ietf.phext, research.phext's in research.phext, the registers' in the
  registers — hosted by **SQ** (`sq host`, REST base /api/v2), the same
  hosting as the rest of the corpus. A node's identity is its (home
  phext, coordinate) address (§2.1); KGOAT addresses across the
  federation by exactly that identity, and an edge may target any
  coordinate in any federated phext.
- **Nothing moves.** ietf.phext keeps its home in the rfcbox programme;
  research.phext in the R&D tracker; the registers in wbic16/human and
  wbic16/agi. Node scrolls sit beside the material in the same phext;
  cross-phext edges do the joining between phexts. (No sidecars, and no
  shadow copies: one canonical home per scroll.)
- **phext.tech is a direct upgrade of the edit.phext.io deployment**, not
  a greenfield host: the same phext-lattice/SQ stack, extended with the
  graph layer and the traversal service. The launch plan (§6) is written
  as that upgrade.
- **Code home:** wbic16/kgoat — the format definition, the builder, the
  traversal service (§3), and the phext.tech surface, in the exocortical
  lineage (libphext underneath; no foreign framework tree).

## 3. Query and traversal

### 3.1 The machinery (for those who go looking)

Three layers, each in the lineage:

1. **SQ** serves the phexts — load, table of contents, select by
   coordinate. Coordinate lookup is layer one's native act.
2. **The KGOAT traversal service** (Rust, on libphext, sibling to
   phext-lattice) loads node scrolls, parses edge lines, folds history,
   and answers graph questions: neighborhood at 1 and n hops, filtered by
   kind, edge type, weight floor, and status; paths between two nodes,
   returned *with their derivations* (§5.3); and **holographic search** —
   enter words, compute their merged coordinate, and return the nodes
   whose addresses neighbor it, nearest first. Holographic search is the
   query form Axiom 1 implies: you do not need to know where a thing is
   filed, because it was never filed — you describe it, and the space
   answers with what's near the description. The traversal service is the
   composition's own machinery — the join made executable; it belongs to
   no single component and serves all three.
3. **phext-lattice** — the worker component (§0.1) — serves the web
   surface on top of the traversal service, as it already serves Phext
   Edit. phext.tech is this deployment upgraded in place (D4, §2.5), not
   a second host beside it. The surface it serves is the religious
   component's (§3.2): scrollspace presenting the graph.

Agents get the same three layers directly — the graph is a Choir tool as
much as a public surface, and SQ's REST is already the corpus's machine
door. There is no separate "API version" of the graph with different
semantics; the surface and the service read the same scrolls.

### 3.2 The visitor's first 60 seconds (Axiom 2 governs)

This is the religious component — scrollspace (§0.1) — presenting the
graph. A visitor to phext.tech who has never heard of phext sees:

- **A single plain entry box**: "Enter anything — a name, a topic, a
  thing." No coordinate field, no kind picker, no instructions about
  dimensions. (Search begins in the same box: if it's here, you find it;
  if it isn't, entering it is how it gets here — entry is write access,
  and write access requires signup, §4.4 and D3.)
- **Results as readable pages, not graph diagrams.** A node renders as a
  page: a plain-language statement of what the thing is; if it bears
  claims, its status in plain words ("verified", "disputed", "not yet
  settled", "a constructed example") with the receipt one click away; its
  strongest connections listed as sentences ("Implements RFC 9110",
  "Derived from…", "Contradicted by…"), each one a link to walk.
- **Walking is clicking.** Every connection is a door to the next page.
  The traversal machinery (§3.1) is doing n-hop work underneath; the
  visitor experiences it as reading linked pages, because that is what
  scrolls are.
- **Counts as receipts, in monospace stamps**: how many things, how many
  connections, when the graph was last folded. Honest numbers, per the
  design language — including the honest scope statement of §1.1.
- **No login to view anything.** Read-only public access (D3): traverse
  and view without a login; **signup is required for write access** —
  entering new things or changing existing ones.

What the visitor never sees unless they go looking: coordinates (they live
in the footer stamp and on an "inspect" depth per page), edge weights as
numbers (rendered as strength of language — "strongly connected" — with
the number on the inspect depth), hop stamps, basis codes. The depth is
real and reachable; it is simply not the door.

### 3.3 The surface in the design language

phext.tech follows Phext, Inc. Design Language v1: dark mode following the
OS preference with a persisted override; Metallic Golden Phext struck flat
on paper white / true black; serif display and body, monospace for stamps;
hard rules, squared corners, no gradients, no glass, no emoji bullets, no
marketing copy a receipt can't back.

Its deterministic variation, from the domain's holographic coordinate
**31.15.17/19.17.19/24.18.24** (computed by the Singular machinery):

- Gold hue shift: (31 + 15 + 24) mod 21 − 10 = **−3°**
- Header rule weight: 2 + (24 mod 3) = **2 px**
- Measure: 720 + (17 mod 5) × 16 = **752 px**
- Tag style: 19 mod 3 = 1 → **filled tags**
- Secondary dominance: 17 mod 2 = 1 → **gold leads**, blue/yellow supports
- Footer stamp: `phext.tech · 31.15.17/19.17.19/24.18.24`


## 4. Ingestion

### 4.1 Seed order — by readiness and edge density

The graph is seeded from corpora that already exist as phexts, in the
order that gives a walkable, assayed graph soonest. Each source contributes
what it is rich in; no source is asked to become something else first.

1. **ietf.phext** — the scale proof, and a finished artifact: built
   2026-11-18, passed through a three-leg independent audit to a final
   READY verdict, hosted and hash-verified (564,589,207 bytes, SHA-256
   9ecd79461f646c84838f697488ebf46d900bf5dd1cfde2b9967db3f6f426d165).
   It is completed prior art that KGOAT *generalizes* — the first
   large-scale graph this programme extracted — never an input still
   under construction. What is in progress is KGOAT itself and its
   phext.tech surface; ietf.phext is not. Its 10,052 document/project
   nodes carry 96,168 references edges and 6,888 implements edges
   *already extracted and stored* by the rfcbox build. Ingestion here is
   a *reading*, not an extraction: the edges exist; KGOAT's builder
   reads them into node scrolls. This source alone makes the graph real
   on day one.
2. **research.phext** — the assay seed. 822 scrolls of concepts and claims
   whose MOAT registry already assigns every claim-bearing unit a status,
   a basis, and an evidence pointer. KGOAT inherits those assignments
   verbatim (§5.1); it does not re-judge them.
3. **The registers** — artifacts.phext (works, with receipts), skills.phext
   (capabilities as works), tasks.phext (works in flight), state.phext
   (events and metrics). These supply the graph's present tense: what
   exists, what is being made, what has been measured.
4. **Group pages** — orgs. People do not enter as nodes at all (D6,
   §5.2): KGOAT is for ideas, not people. Where a work's provenance names
   its maker, the name rides as attribution on the work's scroll — an
   evidence pointer, not a node (§1.2).
5. **The agi corpus / incipit** — the deep reference texts (10,140 scrolls
   at last count). Accessible at launch like every curated phext (D2) —
   scrollspace already walks them; their graph-joining (document and
   concept node scrolls, and their edges) deepens gradually as the
   tracker reads them, not in one bulk pass.

Order rationale, stated once: edges first (ietf), assay second (research),
present tense third (registers), orgs fourth, depth last. The order is
the order of graph-joining work, not of access: under D2 every curated
phext is accessible at launch; under D6 people are not in the order at
all, because they are not in the graph.

### 4.2 The emissary problem, as a design rule

Systems that exchange structured material assign their own identities —
Mutable's phext import, to take the case on record, assigns fresh random
block UUIDs at the border, so only coordinates survive the crossing. Any
KGOAT edge, import, or export that rests on a foreign identity (a UUID, a
database key, another graph's node ID) is therefore broken by construction:
it points at something that will not exist on the other side.

The rule: **all cross-border reference is by coordinate.** A KGOAT node
may carry, as plain scroll content, a note of how a foreign system names
the same thing — a URL, a foreign ID, a DOI — as *data about* the node.
It may never use that name as the node's identity or as an edge's target.
Edges target coordinates. Identities are local; coordinates are the
diplomacy.

### 4.3 How a new source joins

A source joins the graph by meeting four conditions — the same four for
our own future corpora and for anyone else's:

1. **It exists as a phext** (or is converted to one — the phext-upgrade
   path: assess, adapt what gains, keep what must stay). The graph does
   not ingest formats it cannot address.
2. **Every contributed node carries a coordinate in its home phext**,
   placed holographically at entry (Axiom 1) or already resident there.
3. **Its edges are expressed as coordinate references only** (§4.2), in
   the §2.2 line grammar or in a stored form a builder can read into it
   losslessly, as ietf.phext's tables are read.
4. **Claim-bearing material arrives with status and basis, or defaults.**
   A claim arriving unassayed enters as Unknown/ATTRIBUTED — attributed to
   its source, believed by no one yet. There is no path by which imported
   material enters as True on its own say-so.

Joining is a builder pass with a conformance run (§5.4), not a hand edit
and not a merge of files. A source that cannot meet the conditions is
linked to — mentioned, at weight ≤ 1 — not ingested. The graph would
rather hold a weak honest edge than a strong fabricated structure.

### 4.4 Entry by people (Axiom 2 governs)

Everything above describes machines ingesting corpora. The definition —
"organizes everything **you enter**" — describes a person entering things,
and the design for that is deliberately the smallest surface in this spec:

- One plain form: a box for the thing itself (text, a link, a name), an
  optional line for where it came from. No kind picker (the kind is
  inferred from the material and shown, correctable, after), no coordinate
  field (placement is computed), no status questions (a person's own entry
  about their own material enters under the default rules of §4.3, condition 4, and
  anything stronger requires a receipt, which the form invites but never
  demands).
- The organization is the system's job, entirely: holographic placement,
  kind inference, candidate connections found by neighborhood and offered
  as one-click confirmations ("this looks connected to X — it is?"), assay
  defaults. The person judges; the graph files.
- Entry is write access, and write access requires signup (D3, settled
  2026-11-21: "Read-only public access, Signup required for Write
  access"). Viewing never does. The door itself is the worker component's:
  phext-lattice's login-gated editing, the same door edit.phext.io
  already keeps.

## 5. Assay

### 5.1 MOAT discipline, inherited whole

This section is the warrior component (§0.1): tluf. KGOAT does not invent
an epistemics. It inherits the Minimum Viable Oracle discipline that
already governs research.phext — tluf's machinery (wbic16/moat) is what
runs that discipline today, and in the composition it runs over the graph
the same way. What follows is that discipline, stated for the federated
graph:

- Every **claim-bearing unit** — a claim node, and any edge whose existence
  asserts something about the world — carries exactly one **status**
  (True / False / Unknown / Fiction), one **basis** (STIPULATION,
  VERIFIED, REFUTED, ATTRIBUTED, OPEN, SYNTHESIS, ESTIMATE, CONSTRUCTION),
  and an **evidence pointer**. The basis constrains the status exactly as
  in the MVO table: no VERIFIED receipt, no True; a refuted claim is
  False/REFUTED and *kept*, visible as False, never silently deleted.
- **Structural edges are not claims.** part_of and references assert only
  that the graph's sources record a connection; they carry no status
  (§2.2). The discipline bites exactly where assertion lives, and nowhere
  else — assaying everything would assay nothing.
- The graph's **registry** — one entry per claim-bearing unit: coordinate,
  one-line claim, status, basis, evidence pointer — is generated by the
  builder from the scrolls (§2.3), rendered grouped by status, and folded
  into the Home scroll. Expect the honest picture to be mostly Unknown:
  attributed material dominates any young graph, and the counts saying so
  is the fidelity working, not failing.
- **Transitions** follow the MVO legal edges only, each recorded as a
  history event with its receipt (§2.4). Fiction is a first-class status:
  constructions are held as constructions, labelled in plain words on the
  surface ("a constructed example"), and never promoted directly to True.
- The MVO **Lean accuracy proof** (the registry model — total, unique,
  basis-conformant, receipt-backed — machine-checked over research.phext)
  is the pattern for KGOAT's registry assurance. The proof posture is
  settled (D5, Will, 2026-11-21: "Prove what we can"): best-effort from
  v1 — prove what is provable now (the bookkeeping properties, and the
  accuracy properties within reach of the existing model), name the
  remainder as follow-on work in the open, and gate nothing on the proof.
  It is neither deferred wholesale nor a launch requirement. The
  bookkeeping (status/basis/receipt on every claim-bearing unit) is not
  optional either way. The proof makes the bookkeeping incorruptible; the
  receipts make it worth trusting; v1 must have the second from day one.

### 5.2 Scope: KGOAT is for ideas, not people

Settled by D6 (Will, 2026-11-21), verbatim: **"KGOAT is for ideas, not
people."** This is the ground of the format, not an ethics overlay on it.
KGOAT's subject matter is ideas — works, concepts, claims, artifacts, and
their relations. People are not the graph's subject matter at all, and
the families below are empty *because they are out of scope by purpose* —
the same restraint Boardy's doctrine reached as ethics for a graph that
*is* about a person, KGOAT holds as definition for a graph that is not:

- **There are no person nodes.** No kind in §1.2's table names a person,
  and no scroll in the graph is *about* a person. People appear only as
  attribution — a maker's name in a work's provenance lines, an evidence
  pointer (§1.2) — which is a statement about the work, not a node about
  the person.
- **person→person ties: never stored, never derived.** With no person
  nodes there is nothing to tie; and the graph does not answer questions
  about people by derivation either (§5.3). A tie between two people is
  not a fact about ideas, so the format has no place to put one.
- **private_context: not a kind, not an attribute.** What someone keeps
  off the record is not the graph's material — it does not enter the
  structure at all, not as a node, not as a field on a legitimate kind.
- **person→time-of-learning: not stored.** The graph records when *it*
  learned a thing (hop stamps on edges and entries — its own history).
  When a person learned a thing is biography, and biography is out of
  scope.

The empties are stated in the format and checked by the conformance run
(§5.4): an implementation that introduces person nodes or stored person
ties is non-conformant — not because it broke a privacy rule, but because
it built a different graph than the one this format defines. The scope is
the subject matter, stated — not a promise with a hope.

### 5.3 Derivation-graded edges

Some edges are stored; some are **derived** — computed at query time from
stored structure: the path between two nodes, the "these two concepts are
one hop from the same source" observation, the shared-implementations
join between two documents. (Derivations stay inside the graph's subject
matter: no derived edge has a person at either end — §5.2.) Derived edges are where a knowledge graph either earns trust
or becomes a filing cabinet with confidence.

The rule, Boardy's, adopted as format law: **a derived edge is graded on
its derivation, not its landing spot.** Concretely:

- Every derived edge carries its derivation as inspectable content: the
  stored edges and nodes it was computed from, the rule that computed it,
  and the weights along the path. On the surface, a derived connection is
  shown with its reasoning one click away ("connected because both
  implement RFC 9110" — with the implements edges openable).
- A derived edge with a shown derivation outranks an asserted edge with
  none. An asserted edge whose asserter cannot be named is a mentions at
  weight ≤ 1 wearing a costume, and the builder demotes it to exactly
  that.
- Derived edges are **never promoted to stored edges silently**. If a
  derivation is worth keeping, a human or an accountable agent stores it
  as an edge with the derivation as its evidence pointer — an act with an
  author, not a cache with ambitions.

### 5.4 Conformance: Recovery and Invention

Any boot, import, or re-ingestion of a source is scored the Boardy way,
from the graph alone:

- **Recovery (R):** does the result reproduce the dense families (§1.4)
  with the right direction, type, and weight within one step? A boot that
  flips implements into implemented_by-as-storage, or loses the
  references edges' direction, did not read the artifact.
- **Invention (I):** what did the result add where the format is empty
  (§5.2)? Person nodes, stored person ties, private context smuggled in
  as attributes, kinds invented for unenumerated material — each is a
  fabrication, scored against the boot: material outside the format's
  subject matter, presented as if it were the graph's.

Both numbers are reported on every ingestion (§4.3), in the open, next to
the counts. Boardy's prediction is adopted as the format's motto-in-fact:
the second number is the one that separates a format from a filing cabinet.

## 6. The v1 build sequence — composing the three components

Per §0.1, the build is a composition, not three greenfield builds. What
already exists: the worker (phext-lattice, shipped, serving Phext Edit),
the warrior (tluf's assay machinery, running the MVO discipline over
research.phext), and the religious (scrollspace's hosted corpus, walkable
without a login). What the composition still needs: **the graph layer
itself** (§2) — node scrolls in the seed corpora's home phexts, federated
(§2.5); **the builder** extended to read the seed corpora into their
graph layers (§2.3, §4.1); **the traversal service** (§3.1); and
**the phext.tech surface** presenting all three components as one product
(§3.2) — an upgrade of the existing edit.phext.io deployment (D4), not a
new host. The stages below are ordered so each ships independently and the
graph is honest at every stage — nothing below requires a later stage to
be true. Sizing is against the real constraint: a solo builder with an
agent staff, and a hard external date — the No Excuses submission closes
**15 Dec 2026**, with phext.tech as the product's primary home.

- **Stage 0 — this spec, and the schema freeze.** Kind table, edge
  catalogue, line grammar, scope. The schema-touching decisions are all
  ruled (D4 federation, §2.5; D6 scope, §5.2 — §7, frozen 2026-11-21),
  so this stage's decision gate is satisfied. *No code.*
- **Stage 1 — the ietf slice: the first graph layer.** The builder reads
  ietf.phext's stored edges into node scrolls in ietf.phext itself
  (§2.5); the worker hosts them; the religious walks them — search + node
  page + one-hop walk over the RFC graph; the warrior stands behind the
  counts (the ietf edges are structural, so its registry over this slice
  is generated but light). This reuses extraction that already exists and
  is audited — it is a builder and a thin surface, not new extraction.
  *Shippable alone: a walkable knowledge graph with 100K+ real edges, two
  components live and the third in place. It is the first build stage,
  not the boundary of the launch (D2).*
- **Stage 2 — the corpus graph layers: the warrior joins in full.**
  research.phext's concepts and claims get their node scrolls in their
  home phext, carrying their MOAT statuses, assayed by tluf's machinery
  exactly as it assays them today; the registers join as works, events,
  metrics in theirs. The first genuinely layer-two moment lands here:
  **cross-corpus edges** — an RFC node connected to a research concept, a
  bench metric measuring a work — edges that exist in no single source
  phext and could not exist before the federation joined them. After this
  stage, all three components are working in the composition, not just
  present in it. Access is not what this stage adds — every curated phext
  is accessible from launch (D2); this stage adds their graph depth.
- **Stage 3 — entry: the worker's door, opened to people.** The plain form
  of §4.4, signup-gated for write access (D3), riding phext-lattice's
  existing editing: enter a thing, watch it placed, confirm its offered
  connections. The definition's promise, made usable by people who know
  nothing of phext.
- **Stage 4 — derivation and depth.** The warrior's derivation grading as
  a surface feature (derived edges with shown derivations); the
  religious component's hop-time history views ("how this node's record
  changed"); the Recovery/Invention conformance suite run as a standing
  gate on every ingestion; holographic search tuned from real queries.

**Launch-day minimum (15 Dec 2026).** Launch scope is settled (D2):
phext.tech launches with **access to all phexts currently being
curated**. The launch is the upgrade of the edit.phext.io deployment to
phext.tech (D4), presenting the composition at that scope:

1. phext.tech live in the design language — the edit.phext.io deployment
   upgraded in place — with Scroll Space and Phext Edit as the working
   product, and **every currently curated phext accessible** through it:
   the registers, research.phext, the agi corpus, and ietf.phext,
   traversable read-only without a login (D3); signup gates write access.
2. **The KGOAT graph layer present across that scope**: node pages and
   walking live over the federated graph, with the ietf slice — 100K+
   audited edges, the deepest pre-extracted component — searchable and
   walkable as the graph's proof of scale, labelled in plain words as
   what it is, with its real counts (nodes, edges) shown as stamps. The
   ietf slice is a component of the launch, not its whole: the corpus
   slices' graph depth (Stage 2's cross-corpus edges) is the first
   follow-on, and access does not wait for it.
3. the format itself public in wbic16/kgoat: this spec's §§0–2 in public
   form, so the layer-2 claim is inspectable, not asserted.

The floor, if the graph layer slips past the date, is items 1 and 3 with
a KGOAT page stating the seed counts from the rfcbox receipts and the
build order — a promise with receipts about its own absence. Access to
the full curated corpus is not part of what may slip: that is the launch
(D2).

## 7. Decisions — RULED (frozen 2026-11-21)

Naming was never among these — KGOAT is named (§0). All six decisions
below were ruled by Will in the main chat on 2026-11-21 (D1–D5 at 15:46
CST, D6 at 15:47 CST) and are applied throughout this spec. This section
is frozen: each decision stands with its ruling quoted.

- **D1 — the submitted Project.** RULED: "We will submit it simply as The
  Exocortex of 2130." The No Excuses submission is named **The Exocortex
  of 2130**; the three components (worker ⇒ phext-lattice, warrior ⇒
  tluf, religious ⇒ scrollspace) sit inside it. The application draft is
  aligned to the name.
- **D2 — the launch scope.** RULED: "Launch with access to all phexts
  currently being curated." Launch presents the full curated corpus
  (§6, launch-day minimum). This supersedes this spec's earlier
  Stage-1-only launch recommendation: the ietf slice remains a component
  of the launch — its deepest pre-extracted graph — not the whole of it.
- **D3 — entry and access.** RULED: "Read-only public access, Signup
  required for Write access." Public surfaces are read-only without a
  login (the standing Scrollspace rule); signup gates all write access —
  entry and editing — across the components (§§3.2, 4.4). This replaces
  the spec's earlier open-entry leaning (entries landing
  Unknown/ATTRIBUTED pending review as an entry policy; the assay
  defaults for unassayed claims, §4.3, are a separate discipline and
  stand).
- **D4 — topology.** RULED: "Federation of phexts from the start, like
  edit.phext.io - phext.tech is a direct upgrade." The graph is federated
  from the start (§2.5): each phext carries its own graph layer, KGOAT
  addresses across them by (home phext, coordinate) identity, and
  phext.tech is a direct upgrade of the edit.phext.io deployment. This
  supersedes the single-graph.phext v1 design this spec originally
  carried.
- **D5 — the proof posture.** RULED: "Prove what we can." Best-effort
  from v1 (§5.1): prove what is provable now, name the remainder as
  follow-on in the open; the proof is neither deferred wholesale nor a
  launch gate.
- **D6 — scope.** RULED: "KGOAT is for ideas, not people." People are
  not the graph's subject matter (§5.2): there are no person nodes, no
  stored or derived person ties, no private context, no
  person→time-of-learning — empty because out of scope by purpose, not
  as an ethics overlay. Authorship and provenance references to people
  remain what they always were: attribution on a work's scroll, an
  evidence pointer — never a node (§1.2).

## 7A. Decision accounting

How each decision reached its final state. Two rulings did the work: the
component ruling (Will, main chat, 2026-11-21 15:43 CST — worker ⇒
phext-lattice, warrior ⇒ tluf, religious ⇒ scrollspace) and the decision
rulings (2026-11-21 15:46–15:47 CST, quoted in §7).

- **D1 — settled by direct ruling, over ground the component ruling had
  already dissolved.** The component ruling left no separable candidate
  (the editor stack is the worker, the traversal surface is the religious
  component, the platform is the composition); the direct ruling then
  named the submission: **The Exocortex of 2130**. The draft is aligned
  to the name; the components sit inside it.
- **D2 — settled by direct ruling; supersedes a spec recommendation.**
  The ruling ("Launch with access to all phexts currently being
  curated") replaced this spec's Stage-1-only launch recommendation.
  Stage 1 remains the first build stage; it is no longer the launch's
  boundary. The ietf slice is a component of the launch, not the whole.
- **D3 — settled by direct ruling; supersedes the spec's leaning.** The
  component ruling had already answered the feasibility half (entry is
  the worker's existing login-gated door); the direct ruling settled the
  policy: read-only public access, signup required for write access. The
  spec's earlier leaning — entry open with entries landing
  Unknown/ATTRIBUTED pending review — is replaced as an entry policy;
  Unknown/ATTRIBUTED survives only as the assay default for unassayed
  claim-bearing material (§4.3), which is warrior discipline, not access
  policy.
- **D4 — settled by direct ruling; supersedes this spec's v1 design.**
  The original design was one graph.phext for v1 with federation as the
  growth path. The ruling inverts it: federation from the start, on the
  edit.phext.io pattern, with phext.tech a direct upgrade of that
  deployment. §§2.5, 3.1, and 6 are revised to the federated model.
- **D5 — settled by direct ruling.** "Prove what we can": the
  bookkeeping-plus-partial-proof posture of §5.1. Neither posed
  alternative — proof required in v1, proof deferred wholesale — is the
  ruling.
- **D6 — settled by direct ruling.** "KGOAT is for ideas, not people."
  §5.2 is re-grounded from ethics to scope: the empties stand because
  people are outside the format's subject matter by purpose. The person
  kind is removed from §1.2's table; authorship remains as attribution
  and evidence pointers, never as nodes.

---

*Spec ends. Grounding: the MOAT / minimum-viable-oracle skill (status,
basis, legal transitions, the registry discipline); Boardy's spec and boot
manual of 2026-11-18 (typed weighted edges, dense/empty doctrine,
derivation grading, Recovery/Invention); the rfcbox programme record
(ietf.phext counts, fingerprint, audit chain); the corpus audit of
2026-11-16 (register counts); Phext, Inc. Design Language v1 (surface,
§3.3); the Scrollspace rule (view without login, login to edit); the
standing rules on sidecars, registers, and bracket formats. Design only —
no build authorized.*
