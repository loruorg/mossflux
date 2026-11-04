---
description: What does an AI chemistry search MCP tool do? It searches a compound library with exact similarity and substructure queries. Learn to pick one and set it up.
faq:
  - answer: An AI chemistry search MCP tool is a server that exposes molecular search to an agent through the Model Context Protocol. The agent sends a tools/call request with a SMILES or SMARTS query, and the server returns matching molecules. mossflux does this with exact Tanimoto similarity and SMARTS substructure search.
    question: What is an AI chemistry search MCP tool?
  - answer: An agent can run SMARTS substructure search over your own library, because a mossflux database is a SMILES file that you load yourself. A query such as S(=O)(=O)N returns every molecule that contains a sulfonamide. A search that passes its deadline returns an error instead of a partial list.
    question: Can an AI agent run substructure search on my own compound library?
  - answer: The mossflux MCP server costs nothing and needs no account. It is free and open source, licensed under MIT OR Apache-2.0, and there is no paid plan, hosted service or account. The public demo is a small shared server that searches ChEMBL 37.
    question: Does the mossflux MCP server cost anything or need an account?
  - answer: "mossflux similarity search returns every true hit, because it is exact and every answer matches a brute-force scan. That means no false negatives and no false positives: a threshold search returns each molecule whose Tanimoto score is at or above the threshold you set."
    question: Does mossflux similarity search return every true hit?
  - answer: "mossflux parses stereochemistry and ignores it, does no salt or tautomer handling, and its fingerprints do not match RDKit's bit for bit. Canonicalise your SMILES before loading them. Its ECFP4-style fingerprints use mossflux's own bits, so similarity scores differ from RDKit's."
    question: Does mossflux handle stereochemistry or salts, and do its fingerprints match RDKit?
image: /blog/ai-chemistry-search-mcp-tool/featured-102cbc94f3f1.webp
image_alt: "Illustration for AI chemistry search MCP tool: Exact results"
publishDate: "2026-11-04"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/ai-chemistry-search-mcp-tool#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"datePublished\":\"2026-11-04\",\"description\":\"What does an AI chemistry search MCP tool do? It searches a compound library with exact similarity and substructure queries. Learn to pick one and set it up.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"An AI chemistry search MCP tool is a server that exposes molecular search to an agent through the Model Context Protocol. The agent sends a tools/call request with a SMILES or SMARTS query, and the server returns matching molecules. mossflux does this with exact Tanimoto similarity and SMARTS substructure search.\"},\"name\":\"What is an AI chemistry search MCP tool?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"An agent can run SMARTS substructure search over your own library, because a mossflux database is a SMILES file that you load yourself. A query such as S(=O)(=O)N returns every molecule that contains a sulfonamide. A search that passes its deadline returns an error instead of a partial list.\"},\"name\":\"Can an AI agent run substructure search on my own compound library?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The mossflux MCP server costs nothing and needs no account. It is free and open source, licensed under MIT OR Apache-2.0, and there is no paid plan, hosted service or account. The public demo is a small shared server that searches ChEMBL 37.\"},\"name\":\"Does the mossflux MCP server cost anything or need an account?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux similarity search returns every true hit, because it is exact and every answer matches a brute-force scan. That means no false negatives and no false positives: a threshold search returns each molecule whose Tanimoto score is at or above the threshold you set.\"},\"name\":\"Does mossflux similarity search return every true hit?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux parses stereochemistry and ignores it, does no salt or tautomer handling, and its fingerprints do not match RDKit's bit for bit. Canonicalise your SMILES before loading them. Its ECFP4-style fingerprints use mossflux's own bits, so similarity scores differ from RDKit's.\"},\"name\":\"Does mossflux handle stereochemistry or salts, and do its fingerprints match RDKit?\"}],\"name\":\"AI chemistry search MCP tool: Exact results\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/d4J0v2viQQnc84YgExcKROp-fd6SuA4t.webp\"},\"url\":\"https://mossflux.com/blog/ai-chemistry-search-mcp-tool\"},{\"@id\":\"https://mossflux.com/blog/ai-chemistry-search-mcp-tool#article\",\"@type\":\"BlogPosting\",\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com/\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"GitHub - JackKuo666/PubChem-MCP-Server: Enable AI assistants to ...\",\"url\":\"https://github.com/JackKuo666/PubChem-MCP-Server\"},{\"@type\":\"CreativeWork\",\"name\":\"ChemMCP - osu-nlp-group.github.io\",\"url\":\"https://osu-nlp-group.github.io/ChemMCP/\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"PubChem MCP: AI Agent \\u0026 Scientific Chemical Database Access\",\"url\":\"https://mcpmarket.com/server/pubchem-2\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"Tools - Model Context Protocol\",\"url\":\"https://modelcontextprotocol.io/specification/2025-06-18/server/tools\"},{\"@type\":\"CreativeWork\",\"name\":\"Transports - Model Context Protocol\",\"url\":\"https://modelcontextprotocol.io/specification/2025-06-18/basic/transports\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com/#demo\"},{\"@type\":\"CreativeWork\",\"name\":\"Connect Claude Code to tools via MCP\",\"url\":\"https://code.claude.com/docs/en/mcp\"},{\"@type\":\"CreativeWork\",\"name\":\"Model Context Protocol (MCP) | Cursor Docs\",\"url\":\"https://cursor.com/docs/context/mcp\"},{\"@type\":\"CreativeWork\",\"name\":\"For Claude Desktop Users - Model Context Protocol\",\"url\":\"https://modelcontextprotocol.io/quickstart/user\"},{\"@type\":\"CreativeWork\",\"name\":\"loruorg/mossflux on GitHub\",\"url\":\"https://github.com/loruorg/mossflux\"}],\"dateModified\":\"2026-11-04\",\"datePublished\":\"2026-11-04\",\"description\":\"What does an AI chemistry search MCP tool do? It searches a compound library with exact similarity and substructure queries. Learn to pick one and set it up.\",\"headline\":\"AI chemistry search MCP tool: Exact results\",\"image\":\"https://api.birchseek.com/m/d4J0v2viQQnc84YgExcKROp-fd6SuA4t.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/ai-chemistry-search-mcp-tool#webpage\"},\"keywords\":[\"ai chemistry search mcp tool\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/ai-chemistry-search-mcp-tool#webpage\"},\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2373},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"all genetic material of an organism\",\"name\":\"Genome\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Genome\",\"https://www.wikidata.org/wiki/Q7020\",\"https://www.google.com/search?kgmid=/m/037_b\"]},{\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]},{\"@type\":\"Thing\",\"description\":\"process by which new candidate medications are discovered\",\"name\":\"Drug discovery\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Drug_discovery\",\"https://www.wikidata.org/wiki/Q1418791\",\"https://www.google.com/search?kgmid=/m/02r6nx\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"statistical Markov model\",\"name\":\"Hidden Markov model\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hidden_Markov_model\",\"https://www.wikidata.org/wiki/Q176769\",\"https://www.google.com/search?kgmid=/m/0p8wy\"]},{\"@type\":\"Thing\",\"description\":\"digital identifier to identify a certain piece of data, derived from the data by an algorithm\",\"name\":\"Fingerprint\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Fingerprint_(computing)\",\"https://www.wikidata.org/wiki/Q3745713\"]},{\"@type\":\"Thing\",\"description\":\"specialized artificial neural networks that are designed for tasks whose inputs are graphs\",\"name\":\"Graph neural network\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Graph_neural_network\",\"https://www.wikidata.org/wiki/Q107750691\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - mcp
  - cheminformatics
  - substructure search
  - similarity search
  - mossflux
  - ai agents
title: "AI chemistry search MCP tool: Exact results"
---

**An ai chemistry search mcp tool is a server that gives an agent tools to search a compound library, and the useful ones return exact results. [mossflux](https://mossflux.com/) is the open-source Rust option: [five tools](https://mossflux.com/docs/api) (`list_databases`, `similarity_search`, `substructure_search`, `molecule_properties`, `get_molecule`) over stdio or streamable HTTP, with a public demo on ChEMBL 37 (2.9 million molecules).**

You want an agent to answer "which molecules contain a sulfonamide?" or "what is close to aspirin?" over a compound library you own. The first ai chemistry search mcp tools you find are [PubChem wrappers](https://github.com/JackKuo666/PubChem-MCP-Server), which reach PubChem's database, and toolkits like [ChemMCP](https://osu-nlp-group.github.io/ChemMCP/), built for molecular analysis and property prediction. Neither searches your collection.

Agent builders, cheminformatics engineers and platform engineers all run into this gap when they look for an ai chemistry search mcp tool. After reading, you can pick the right kind of tool, call it, connect it to a client and plan around its limits.

> **Key Takeaways**
> - Pick by question: a PubChem wrapper for a named compound, ChemMCP for one molecule, mossflux for every match in your collection.
> - REST, gRPC and MCP share one engine, so hits you check with `curl` are the rows an agent receives.
> - Budget about 2.5 KB of RAM per molecule, because the index is built in memory and not saved to disk.
> - Canonicalise salts, tautomers and stereo before loading, and expect fingerprint bits that differ from RDKit's.

## What does an AI chemistry search MCP tool actually search?

An ai chemistry search mcp tool runs one of three operations: similarity search, substructure search, or a name or identifier lookup. Each takes a different input and gives a different guarantee.

![A light beam scans a field of dark molecules, highlighting a selection of exact green molecular structures.](/blog/ai-chemistry-search-mcp-tool/image-1-07274942050f.webp)

Similarity search compares fingerprints. A fingerprint is a row of bits, each marking a structural feature, and the Tanimoto score is the number of bits two fingerprints share divided by the number set in either.

A threshold search returns every molecule with a score of at least t, and a kNN search returns the k nearest. A `score == 1.0` means equal bits, not the same compound.

Substructure search tests for a pattern. The search returns every molecule in which that pattern appears as a subgraph.

A SMILES string is also a valid SMARTS, so a plain structure works as a pattern.

A name or identifier lookup returns one known record. The [PubChem MCP server README](https://github.com/JackKuo666/PubChem-MCP-Server) says it "allows AI models to search for chemical compounds and access their detailed information in a programmatic way." You give it one name or ID and get one record back, with no score and no library scan.

Two queries on the same library show the difference:

- Similarity query, aspirin `CC(=O)Oc1ccccc1C(=O)O`: which molecules have a fingerprint close to aspirin's.
- Substructure query, sulfonamide `S(=O)(=O)N`: which molecules contain a sulfur with two oxygens bonded to a nitrogen, wherever that group sits in the molecule.

The first ranks by closeness of bits. The second tests for presence of a pattern, so there's no score.

For substructure search, mossflux's own parsers, perception, screen and matcher run in a fixed order: parse, perceive, path screen (a superset filter), census, VF2 verify. The [design page](https://mossflux.com/docs/design) describes each stage.

## PubChem wrapper, ChemMCP or a search engine: which MCP tool fits?

The right AI chemistry search MCP tool depends on whether your question names a compound, concerns one molecule, or asks for every match in a collection.

| | PubChem wrapper | Per-molecule toolkit (ChemMCP) | Library search engine (mossflux) |
| --- | --- | --- | --- |
| What it searches | PubChem records | No library; works on a molecule you give it | A SMILES file you load |
| Where the data lives | Remote PubChem service | Wherever the tools run | In memory on your machine or server |
| Typical question | "What is this compound called, and what is its CID?" | "What are this molecule's properties?" | "Which molecules contain a sulfonamide?" |
| Result guarantee | Whatever PubChem returns | Whatever each tool computes | Same rows as a brute-force scan |
| What it cannot do | Search your private collection | Return every match across a collection | Name lookup, per-molecule modelling |

A PubChem wrapper sends every call to a remote service. The [PubChem MCP server's tools](https://github.com/JackKuo666/PubChem-MCP-Server) cover search by name, by SMILES, by CID and an advanced search.

One [MCP Market listing](https://mcpmarket.com/server/pubchem-2) describes the wrapper as providing a "rate-limited interface" over the PubChem PUG REST API, so the PUG REST usage policy on request rate applies. Looping over a whole collection through it is a poor fit.

ChemMCP works on one molecule at a time. The [ChemMCP project page](https://osu-nlp-group.github.io/ChemMCP/) describes "an easy-to-use and extensible chemistry toolkit for LLMs and AI assistants, compatible with the Model Context Protocol (MCP)". It lists molecular analysis, property prediction and reaction synthesis, which are calculations on a molecule you supply.

mossflux searches a SMILES file you load, such as ChEMBL or an in-house library. [Its docs](https://mossflux.com/docs/) state that every answer is exactly what a brute-force scan returns.

mossflux doesn't look up names or do per-molecule modelling. Its `molecule_properties` tool returns formula, weight and counts only, as described in the [API docs](https://mossflux.com/docs/api).

Match each kind to its question:

- A named compound, such as aspirin by name or a CID you already hold, is a lookup. Use the PubChem wrapper, since PubChem holds the record.
- A one-molecule calculation concerns a single structure with no collection involved. Use a toolkit such as ChemMCP.
- "Find every molecule in my collection that matches" calls for mossflux. A similarity query like `CC(=O)Oc1ccccc1C(=O)O` or a substructure query like `S(=O)(=O)N` runs over your own file.

One agent can have all three configured side by side. You don't have to pick just one ai chemistry search mcp tool.

## Why must an AI chemistry search MCP tool be exact?

An AI chemistry search MCP tool must be exact because an agent can't look at a hit list and sense that it's incomplete.

A chemist scanning a list of sulfonamides may notice that a familiar scaffold is missing. An agent reads whatever comes back as the whole answer, so a silent miss becomes a confident wrong statement.

mossflux treats exactness as a design contract. Its [correctness page](https://mossflux.com/docs/correctness) says every result matches a brute-force scan, with no false negatives and no false positives. Four properties back that contract:

- A prune rejects only on a proven bound.
- A substructure search that runs past its deadline returns an error, never a partial list.
- A limit refuses a request. It never changes an answer.
- The substructure engine is tested against RDKit, and 114 of 114 queries agree in the RDKit differential.

One engine also sits behind all three transports. The [API docs](https://mossflux.com/docs/api) say that `mossflux serve` "answers the same searches over REST, MCP and gRPC," so REST, gRPC and MCP cannot disagree about a result.

In practice, you'll prototype a query with `curl` against the REST endpoint and check the hits by eye. An agent given the same engine over MCP then gets the same rows you saw.

## What tools and parameters does a chemistry search MCP server expose?

The mossflux chemistry search MCP server exposes five tools, and each one maps to an endpoint of the same engine. The [mossflux API reference](https://mossflux.com/docs/api) lists them with their fields.

A client then runs a tool with `tools/call`.

1. `list_databases`: returns the served databases, so an agent can pick a `db` name. REST: `GET /v1/databases`.
2. `similarity_search`: Tanimoto search with `smiles`, `threshold`, `k`, `limit` and `db`. Use `threshold` for every molecule at or above a score, or `k` for the nearest. REST: `POST /v1/similarity`.
3. `substructure_search`: SMARTS search with `smarts`, `limit`, `offset`, `timeout_ms` and `db`. `offset` pages through hits, and `timeout_ms` sets the deadline. REST: `POST /v1/substructure`.
4. `molecule_properties`: returns formula, weight and counts for a molecule you supply. REST: `POST /v1/molecule`.
5. `get_molecule`: fetches one record by `db` and `id`. REST: `GET /v1/databases/{db}/molecules/{id}`.

The MCP arguments use the same names as the REST fields.

A similarity search for aspirin takes its `db` value from `list_databases`. The threshold and limit are example inputs.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "similarity_search",
    "arguments": {
      "smiles": "CC(=O)Oc1ccccc1C(=O)O",
      "threshold": 0.7,
      "limit": 5,
      "db": "<database name from list_databases>"
    }
  }
}
```

The result wraps the REST response body in an MCP `content` item:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      { "type": "text", "text": "<hit list: the JSON body that POST /v1/similarity returns>" }
    ],
    "isError": false
  }
}
```

The sulfonamide query `S(=O)(=O)N` goes through `substructure_search`, with `timeout_ms` set to the 10 s default.

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "substructure_search",
    "arguments": {
      "smarts": "S(=O)(=O)N",
      "limit": 10,
      "offset": 0,
      "timeout_ms": 10000,
      "db": "<database name from list_databases>"
    }
  }
}
```

Its result has the same shape:

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "content": [
      { "type": "text", "text": "<hit list: the JSON body that POST /v1/substructure returns>" }
    ],
    "isError": false
  }
}
```

Over REST, the same arguments object goes to the endpoint with `curl -s -X POST https://api.mossflux.com/v1/similarity -H 'Content-Type: application/json' -d @body.json`.

## How do you add a chemistry search MCP tool to Claude Code, Claude Desktop and Cursor?

You add a chemistry search MCP tool to Claude Code, Claude Desktop or Cursor by registering mossflux as a remote URL or as a local stdio command. The [MCP transports specification](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports) defines two standard transports, stdio and Streamable HTTP, and Streamable HTTP replaced the earlier HTTP+SSE transport. mossflux speaks both.

The quickest route is the demo over HTTP. The public demo at `https://api.mossflux.com/mcp` searches ChEMBL 37 on one small 4-thread server. It's a demo, with no hosted service, account or plan behind it, so don't build a workflow on it.

In [Claude Code](https://code.claude.com/docs/en/mcp), the documented form is `claude mcp add --transport http <name> <url>`:

```sh
claude mcp add --transport http mossflux https://api.mossflux.com/mcp
```

[Cursor reads remote servers](https://cursor.com/docs/context/mcp) from `mcp.json`. Add an entry under `mcpServers` with a `url` key:

```json
{"mcpServers": {"mossflux": {"url": "https://api.mossflux.com/mcp"}}}
```

Claude Desktop takes remote servers through its custom connectors setting, not its JSON file. The connection works when the five tools appear in the client. [Try the demo](https://mossflux.com/#demo) first.

To run mossflux locally over stdio, build it yourself.

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

`target-cpu=native` turns on hardware popcount, and the binary then runs only on CPUs like the build machine. Don't copy it to a different host.

[Claude Desktop registers a stdio server](https://modelcontextprotocol.io/quickstart/user) in `claude_desktop_config.json` with `command` and `args` under `mcpServers`. Cursor uses the same two keys in its own `mcp.json`:

```json
{"mcpServers": {"mossflux": {"command": "mossflux", "args": ["mcp"]}}}
```

Append your database flags to `args`; the [mossflux CLI page](https://mossflux.com/docs/cli) lists them.

## What are the limits of this AI chemistry search MCP tool?

This chemistry search MCP tool searches one SMILES file that you load as a database, within fixed limits on results, input, time, memory and chemistry.

- The public demo searches ChEMBL 37, which holds 2,896,304 molecules, according to the [deploy documentation](https://mossflux.com/docs/deploy). The [mossflux site](https://mossflux.com/) lists the demo data as ChEMBL 37 under CC BY-SA 3.0.
- A request returns at most 10,000 hits, per the [API reference](https://mossflux.com/docs/api).
- 4 searches run at once by default, and further requests wait.
- The [security page](https://mossflux.com/docs/security) sets input sizes: SMILES may have up to 65,534 atoms and SMARTS up to 2,048. A query may be up to 4,096 bytes and a request body up to 64 KiB. Malformed or oversized input is an error with a byte position or a line number.
- The index is built in memory at start and is not saved to disk. ChEMBL 37 becomes a 5.1 GB index, about 2.5 KB of RAM per molecule. The same deploy page covers sizing.
- The server computes its own 2048-bit ECFP4-style bits. They are not RDKit-compatible.
- Chirality is parsed and ignored, and there is no salt or tautomer handling, as the [design notes](https://mossflux.com/docs/design) state. Canonicalise salts, tautomers and stereo before you load the file.
- mossflux takes no SDF, InChI or 3D input, keeps no persisted index and supports no deletes, as the [mossflux docs](https://mossflux.com/docs/) list.

## Frequently asked questions

### What is an AI chemistry search MCP tool?

An AI chemistry search MCP tool is a server that exposes molecular search to an agent through the Model Context Protocol. The agent sends a `tools/call` request with a SMILES or SMARTS query, and the server returns matching molecules. mossflux does this with exact Tanimoto similarity and SMARTS substructure search.

### Can an AI agent run substructure search on my own compound library?

An agent can run SMARTS substructure search over your own library, because a mossflux database is a SMILES file that you load yourself. A query such as `S(=O)(=O)N` returns every molecule that contains a sulfonamide.

### Does the mossflux MCP server cost anything or need an account?

The mossflux MCP server is [free and open source](https://github.com/loruorg/mossflux), under MIT OR Apache-2.0. There is no paid plan, hosted service or account. The public demo is a small shared server that searches ChEMBL 37.

### Does mossflux similarity search return every true hit?

mossflux similarity search is exact, and [every answer matches a brute-force scan](https://mossflux.com/docs/). That means no false negatives and no false positives: a threshold search returns each molecule whose Tanimoto score is at or above the threshold you set.

### Does mossflux handle stereochemistry or salts, and do its fingerprints match RDKit?

mossflux parses stereochemistry and ignores it, and it does no salt or tautomer handling, so canonicalise your SMILES before loading them. The fingerprints are mossflux's own and don't match RDKit bit for bit, as the [API docs](https://mossflux.com/docs/api) state.

## How do you start with an AI chemistry search MCP tool?

Today, add the demo and run two queries: aspirin, `CC(=O)Oc1ccccc1C(=O)O`, for similarity and `S(=O)(=O)N` for sulfonamide substructure.

```sh
claude mcp add --transport http mossflux https://api.mossflux.com/mcp
```

This week, install the [CLI](https://mossflux.com/docs/cli) and run `mossflux mcp` over stdio on a SMILES file of your own.

Then run `mossflux serve` with your databases.

## Sources

[mossflux · exact similarity search at billions of molecules…](https://mossflux.com/) · [API · mossflux docs](https://mossflux.com/docs/api) · [GitHub - JackKuo666/PubChem-MCP-Server: Enable AI…](https://github.com/JackKuo666/PubChem-MCP-Server) · [ChemMCP - osu-nlp-group.github.io](https://osu-nlp-group.github.io/ChemMCP/) · [Design · mossflux docs](https://mossflux.com/docs/design) · [mossflux docs](https://mossflux.com/docs/) · [mossflux docs](https://mossflux.com/docs/correctness) · [Tools - Model Context Protocol](https://modelcontextprotocol.io/specification/2025-06-18/server/tools). Every other source is linked where the article cites it.
