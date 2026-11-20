---
description: How does chemistry search via MCP work? An agent calls tools on a server for exact Tanimoto and SMARTS searches over your SMILES file. Learn setup and limits.
faq:
  - answer: Start mossflux mcp for stdio, or point your client at the /mcp endpoint of mossflux serve for streamable HTTP. Then call similarity_search with a SMILES or substructure_search with a SMARTS. Add your own SMILES file with mossflux serve --db NAME=PATH. Results match a brute-force scan.
    question: How do I start chemistry search via MCP with mossflux?
  - answer: SMARTS substructure search works through MCP using the substructure_search tool, which takes a SMARTS pattern. Any SMILES, such as CC(=O)Oc1ccccc1C(=O)O for aspirin, is also a valid SMARTS, so you can paste a plain SMILES as the query. The tool returns every molecule that contains the pattern.
    question: Does SMARTS substructure search work through MCP?
  - answer: A timed-out substructure search returns an error and never a partial list. The server default deadline is 10 s and the maximum is 60 s. If you hit the limit, narrow the SMARTS pattern so the search finishes before the deadline.
    question: What happens when a substructure search times out?
  - answer: "The fingerprints are mossflux's own and are not RDKit-compatible bit for bit, so the bits will not line up with RDKit's. Chirality is parsed and ignored, and there is no salt or tautomer handling. Canonicalise your SMILES before loading them, and handle salts and tautomers yourself beforehand."
    question: Do the fingerprints match RDKit, and are stereochemistry, salts and tautomers handled?
  - answer: You need neither an account nor a hosted service to use mossflux. It is free and open source under MIT OR Apache-2.0, with no paid plan and no accounts. A public demo searches ChEMBL 37 if you want to try it first.
    question: Do I need an account or a hosted service?
image: /blog/chemistry-search-mcp/featured-4dc12f1ccbbe.webp
image_alt: "Illustration for Chemistry search via MCP: Exact hits, your data"
publishDate: "2026-11-20"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/chemistry-search-mcp#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q278487\",\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q11173\",\"@type\":\"Thing\",\"description\":\"pure chemical substance consisting of two or more different chemical elements\",\"name\":\"Chemical compound\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Chemical_compound\",\"https://www.wikidata.org/wiki/Q11173\",\"https://www.google.com/search?kgmid=/m/0129f4\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q133436854\",\"@type\":\"Thing\",\"description\":\"protocol for applications to provide context to LLMs\",\"name\":\"Model Context Protocol\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Model_Context_Protocol\",\"https://www.wikidata.org/wiki/Q133436854\",\"https://www.google.com/search?kgmid=/g/11x5hnm0vb\"]}],\"datePublished\":\"2026-11-20\",\"description\":\"How does chemistry search via MCP work? An agent calls tools on a server for exact Tanimoto and SMARTS searches over your SMILES file. Learn setup and limits.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Start mossflux mcp for stdio, or point your client at the /mcp endpoint of mossflux serve for streamable HTTP. Then call similarity_search with a SMILES or substructure_search with a SMARTS. Add your own SMILES file with mossflux serve --db NAME=PATH. Results match a brute-force scan.\"},\"name\":\"How do I start chemistry search via MCP with mossflux?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"SMARTS substructure search works through MCP using the substructure_search tool, which takes a SMARTS pattern. Any SMILES, such as CC(=O)Oc1ccccc1C(=O)O for aspirin, is also a valid SMARTS, so you can paste a plain SMILES as the query. The tool returns every molecule that contains the pattern.\"},\"name\":\"Does SMARTS substructure search work through MCP?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A timed-out substructure search returns an error and never a partial list. The server default deadline is 10 s and the maximum is 60 s. If you hit the limit, narrow the SMARTS pattern so the search finishes before the deadline.\"},\"name\":\"What happens when a substructure search times out?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The fingerprints are mossflux's own and are not RDKit-compatible bit for bit, so the bits will not line up with RDKit's. Chirality is parsed and ignored, and there is no salt or tautomer handling. Canonicalise your SMILES before loading them, and handle salts and tautomers yourself beforehand.\"},\"name\":\"Do the fingerprints match RDKit, and are stereochemistry, salts and tautomers handled?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"You need neither an account nor a hosted service to use mossflux. It is free and open source under MIT OR Apache-2.0, with no paid plan and no accounts. A public demo searches ChEMBL 37 if you want to try it first.\"},\"name\":\"Do I need an account or a hosted service?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q118876059\",\"@type\":\"Thing\",\"description\":\"large language model family developed by Anthropic\",\"name\":\"Claude\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Claude_(AI)\",\"https://www.wikidata.org/wiki/Q118876059\",\"https://www.google.com/search?kgmid=/g/11kq5ghr35\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q466769\",\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q364\",\"@type\":\"Organization\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q2063\",\"@type\":\"Thing\",\"description\":\"text-based open standard designed for human-readable data interchange\",\"name\":\"JSON\",\"sameAs\":[\"https://en.wikipedia.org/wiki/JSON\",\"https://www.wikidata.org/wiki/Q2063\",\"https://www.google.com/search?kgmid=/m/05cntt\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q18216\",\"@type\":\"Thing\",\"description\":\"medication used to treat pain and decrease the risk of heart disease\",\"name\":\"Aspirin\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Aspirin\",\"https://www.wikidata.org/wiki/Q18216\",\"https://www.google.com/search?kgmid=/m/0qkc\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q8777\",\"@type\":\"Thing\",\"description\":\"application-layer protocol for transferring web resources\",\"name\":\"HTTP\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HTTP\",\"https://www.wikidata.org/wiki/Q8777\",\"https://www.google.com/search?kgmid=/m/03hgt\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q334661\",\"@type\":\"Thing\",\"description\":\"permissive free software license\",\"name\":\"MIT License\",\"sameAs\":[\"https://en.wikipedia.org/wiki/MIT_License\",\"https://www.wikidata.org/wiki/Q334661\",\"https://www.google.com/search?kgmid=/m/04vpm\"]}],\"name\":\"Chemistry search via MCP: Exact hits, your data\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/NSifJPfxcgcwVxE9yoGaoQ2SBAHI-L_d.webp\"},\"url\":\"https://mossflux.com/blog/chemistry-search-mcp\"},{\"@id\":\"https://mossflux.com/blog/chemistry-search-mcp#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q278487\"},{\"@id\":\"https://www.wikidata.org/wiki/Q11173\"},{\"@id\":\"https://www.wikidata.org/wiki/Q133436854\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"GitHub - cyanheads/pubchem-mcp-server: Search the PubChem chemical ...\",\"url\":\"https://github.com/cyanheads/pubchem-mcp-server\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com/#demo\"},{\"@type\":\"CreativeWork\",\"name\":\"Transports - Model Context Protocol\",\"url\":\"https://modelcontextprotocol.io/specification/2025-06-18/basic/transports\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"Tools - Model Context Protocol\",\"url\":\"https://modelcontextprotocol.io/specification/2025-06-18/server/tools\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"loruorg/mossflux\",\"url\":\"https://github.com/loruorg/mossflux\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com/\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/#get\"}],\"dateModified\":\"2026-11-20\",\"datePublished\":\"2026-11-20\",\"description\":\"How does chemistry search via MCP work? An agent calls tools on a server for exact Tanimoto and SMARTS searches over your SMILES file. Learn setup and limits.\",\"headline\":\"Chemistry search via MCP: Exact hits, your data\",\"image\":\"https://api.birchseek.com/m/NSifJPfxcgcwVxE9yoGaoQ2SBAHI-L_d.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/chemistry-search-mcp#webpage\"},\"keywords\":[\"chemistry search via mcp\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/chemistry-search-mcp#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q118876059\"},{\"@id\":\"https://www.wikidata.org/wiki/Q466769\"},{\"@id\":\"https://www.wikidata.org/wiki/Q364\"},{\"@id\":\"https://www.wikidata.org/wiki/Q2063\"},{\"@id\":\"https://www.wikidata.org/wiki/Q18216\"},{\"@id\":\"https://www.wikidata.org/wiki/Q8777\"},{\"@id\":\"https://www.wikidata.org/wiki/Q334661\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2195},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"(as a form of proximity search (metric space)) optimization problem of finding the point in a given set that is closest (or most similar) to a given point\",\"name\":\"Nearest neighbor search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Nearest_neighbor_search\",\"https://www.wikidata.org/wiki/Q608751\",\"https://www.google.com/search?kgmid=/m/025ytbk\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"group of atoms in a molecule which imparts specific chemical properties\",\"name\":\"Functional group\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Functional_group\",\"https://www.wikidata.org/wiki/Q170409\",\"https://www.google.com/search?kgmid=/m/02ydd\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"number of nonzero symbols in a string\",\"name\":\"Hamming weight\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hamming_weight\",\"https://www.wikidata.org/wiki/Q5645805\",\"https://www.google.com/search?kgmid=/m/048nw2\"]},{\"@type\":\"Thing\",\"description\":\"digital identifier to identify a certain piece of data, derived from the data by an algorithm\",\"name\":\"Fingerprint\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Fingerprint_(computing)\",\"https://www.wikidata.org/wiki/Q3745713\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - mcp
  - cheminformatics
  - substructure search
  - tanimoto similarity
  - smiles
  - ai agents
title: "Chemistry search via MCP: Exact hits, your data"
---

**An agent does chemistry search via MCP by calling search tools on a server that speaks the Model Context Protocol. [mossflux serve](https://mossflux.com/docs/api) answers exact Tanimoto similarity and SMARTS substructure searches over MCP, and [the mossflux docs](https://mossflux.com/docs/) say every answer is exactly what a brute-force scan returns, with no false negatives or false positives.**

If an agent's chemistry search via MCP is a proxy to PubChem, it searches PubChem and nothing else. [The servers from cyanheads, JackKuo666 and BioContext](https://github.com/cyanheads/pubchem-mcp-server) each connect an agent to PubChem's chemical database. Your in-house library is invisible to them, and you can't check a result list against a brute-force scan.

A question like which of your own compounds contain `S(=O)(=O)N` never reaches PubChem's data. With mossflux you can connect a client, run a first query, read the limits and point the agent at your own SMILES file.

> **Key Takeaways**
> - [Substructure deadline](https://mossflux.com/docs/api): server default 10 s, maximum 60 s. Past it the search returns an error, never a partial list.
> - Each request is capped at 10,000 hits, and 4 searches run at once by default.
> - Similarity scores use mossflux's own ECFP4-style fingerprint bits, which are not RDKit-compatible.
> - [Serving ChEMBL 37](https://mossflux.com/docs/deploy) (2,896,304 molecules) in memory takes a 5.1 GB index, about 2.5 KB of RAM per molecule.

## What is chemistry search via MCP?

Chemistry search via MCP means an AI agent calls molecular search tools through the Model Context Protocol instead of writing its own code. The agent gets structured results back from the server.

![A stylized illustration showing molecular models being filtered into a precise stacked dataset.](/blog/chemistry-search-mcp/image-1-a1b401477a97.webp)

Most servers for chemistry search via MCP are PubChem front ends. The [cyanheads README](https://github.com/cyanheads/pubchem-mcp-server) describes its server as one that lets you "Search the PubChem chemical database for compounds, properties, safety data, bioactivity, cross-references, and entity summaries via MCP."

The server exposes tools such as similarity and substructure search over stdio or streamable HTTP. Every answer comes from PubChem's records, and it isn't your data.

mossflux is different: it runs chemistry search via MCP itself. If you serve an in-house SMILES file, the agent's query runs inside the mossflux process against that file, not against a public database.

Ask a PubChem server for compounds like aspirin (`CC(=O)Oc1ccccc1C(=O)O`) and the lookup reaches PubChem's compound records. Ask mossflux the same question and it scores your own rows by Tanimoto.

It's one engine behind every transport. The [API docs](https://mossflux.com/docs/api) say that "mossflux serve answers the same searches over REST, MCP and gRPC." REST, gRPC and MCP call the same code, so an agent's answer cannot differ from the API's.

The [design page](https://mossflux.com/docs/design) describes the similarity and substructure engines behind it.

## How do you set up chemistry search via MCP with stdio or HTTP?

You set up chemistry search via MCP by registering a mossflux server with your client over streamable HTTP (the public demo or your own server) or over stdio (a local binary). The [MCP specification](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports) defines these two as its standard transports.

1. Try the public demo over HTTP. Run this in Claude Code:

   ```sh
   claude mcp add --transport http mossflux https://api.mossflux.com/mcp
   ```

2. Install the CLI for local use.

   ```sh
   RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
   ```

 The `target-cpu=native` build runs only on CPUs like the build machine, so don't copy the binary to different hardware.

3. Register the stdio server. Add this entry to your client's MCP configuration (the `mcpServers` shape most desktop clients use):

   ```json
   {
     "mcpServers": {
       "mossflux": {
         "command": "mossflux",
         "args": ["mcp"]
       }
     }
   }
   ```

4. Or serve your own library over HTTP. Start `mossflux serve --db chembl=chembl_37.smi` (the name and path are yours), then register `http://localhost:8080/mcp` with the same `claude mcp add --transport http` command.

 The server builds its index in memory at start.

5. Run a first query. Ask the agent to call `similarity_search` with aspirin, `CC(=O)Oc1ccccc1C(=O)O`. The same engine answers REST, so you can check the hits with curl:

   ```sh
   curl -s -X POST http://localhost:8080/v1/similarity \
     -H 'content-type: application/json' \
     -d '{"smiles":"CC(=O)Oc1ccccc1C(=O)O","threshold":0.7,"limit":5,"db":"chembl"}'
   ```

 Success is a JSON answer with at most 5 hits. The [API reference](https://mossflux.com/docs/api) lists each tool's arguments and result fields, and MCP returns the same hits as REST.

[Try the demo](https://mossflux.com/#demo) first. It searches ChEMBL 37 on one small 4-thread server, so use it to try the tools, not for production load. Once the tools behave as you'd expect, move chemistry search via MCP to your own file.

## Which tools does chemistry search via MCP expose?

A mossflux server exposes five MCP tools: `list_databases`, `similarity_search`, `substructure_search`, `molecule_properties` and `get_molecule`. The [API reference](https://mossflux.com/docs/api) lists their fields, which mirror the REST fields of the same searches.

1. `list_databases` returns the databases the server has loaded, each one a served SMILES file. Use a returned name as the `db` value in the search calls.

2. `similarity_search` runs an exact Tanimoto search with the fields `smiles`, `threshold`, `k`, `limit` and `db`. A threshold search returns every molecule at or above the score, and a kNN search returns the `k` nearest.

3. `substructure_search` matches a SMARTS query with the fields `smarts`, `limit`, `offset`, `timeout_ms` and `db`. Page through large result sets with `offset`.

4. `molecule_properties` returns the formula, weight and counts for a molecule.

5. `get_molecule` fetches one molecule by id from a named database.

 A query can be 4,096 bytes and a request body 64 KiB.

Input outside those bounds is an error, not a truncated search.

PubChem proxy servers cover a different scope. The [cyanheads server](https://github.com/cyanheads/pubchem-mcp-server), for example, lists "10 Tools" that search the PubChem chemical database by name, formula or structure. The five mossflux tools search a file you load yourself.

| Question | PubChem proxy | mossflux |
|---|---|---|
| Which data is searched? | PubChem's database | A SMILES file you load |
| Where does the search run? | PubChem | The mossflux process |
| Can you check hits against a brute-force scan? | No | Yes |

## What do similarity and substructure search return?

Similarity search returns every molecule whose Tanimoto score against the query reaches a threshold, or the k nearest by score. Substructure search returns every molecule that contains the SMARTS pattern as a subgraph. In mossflux both return exactly what a brute-force scan returns, according to [the mossflux docs](https://mossflux.com/docs/), with no false negatives and no false positives.

Tanimoto divides the bits set in both fingerprints by the bits set in either. The [API docs](https://mossflux.com/docs/api) say mossflux computes it over its own 2048-bit ECFP4-style fingerprints. These bits don't match RDKit or other toolkits, so a score from one tool won't equal a score from another.

Threshold search returns every molecule with a score of at least t, so the hit count depends on your data. The [design notes](https://mossflux.com/docs/design) say kNN returns the k nearest, ranked by exact fraction. Ties go to the lower popcount, then the earlier row.

A SMILES is a valid SMARTS, so `c1ccncc1` works as a query. mossflux also reads recursive `$(...)` and the `D` (explicit connections), `X` (total connections), `H` (hydrogens) and `v` (valence) primitives.

Atom maps and chirality parse and are ignored, so a query can't tell R from S.

The project tests this against a brute-force oracle and against RDKit, where 114 of 114 queries agree, as the [correctness page](https://mossflux.com/docs/correctness) records.

### Worked example: aspirin and the sulfonamide

To find neighbours of aspirin, call `similarity_search` with the SMILES and a threshold. `chembl` is an example name: use one returned by `list_databases`.

```json
{
  "name": "similarity_search",
  "arguments": {
    "smiles": "CC(=O)Oc1ccccc1C(=O)O",
    "threshold": 0.7,
    "limit": 20,
    "db": "chembl"
  }
}
```

Replace `threshold` with `k` (for example 10) to get the 10 nearest instead. Every hit carries its exact Tanimoto score, and `limit` caps the list per request.

To find every molecule with a sulfonamide group, call `substructure_search`:

```json
{
  "name": "substructure_search",
  "arguments": {
    "smarts": "S(=O)(=O)N",
    "limit": 100,
    "offset": 0,
    "timeout_ms": 10000,
    "db": "chembl"
  }
}
```

The response lists matching molecules from the database, with `limit` and `offset` paging through them. A `timeout_ms` of 10000 matches the server's default deadline.

## What limits and errors will chemistry search via MCP hit?

An agent on a mossflux server meets five limits, and each one refuses a request instead of changing an answer.

1. Hit cap. One request returns at most 10,000 hits, and the `limit` field sets a lower cap. In the CLI, `--limit` caps only the printed hits, and the counts stay exact, per the [API reference](https://mossflux.com/docs/api).
2. Offset pagination. `substructure_search` takes `offset` beside `limit`, so an agent reads a long hit list one page at a time.
3. Substructure deadline. The server default is 10 s and the maximum is 60 s, and `timeout_ms` sets the deadline for one request.
4. Concurrency. The server runs 4 searches at once by default. A fifth request waits for a free slot.
5. Tool errors. The [MCP specification](https://modelcontextprotocol.io/specification/2025-06-18/server/tools) says tool execution errors are reported inside the tool result, not as protocol errors. A deadline arrives that way.

The exactness contract sits on top of these limits. A limit refuses and never changes an answer, and a timed-out search returns an error, never a partial list.

The [correctness page](https://mossflux.com/docs/correctness) defines what "exact" means in mossflux and the tests behind it.

PubChem proxies behave differently. The [cyanheads server](https://github.com/cyanheads/pubchem-mcp-server) documents `totalFoundAtLeast` saturation floors and an offset ceiling. Those are documented behaviours of a proxy for a hosted API, where a reported total can be a floor. A mossflux count is never a floor.

Did a search fail on its deadline? Rather than resending it, retry in one of two ways. Tighten the SMARTS, for example by adding a ring or a heteroatom the target must contain. Or raise `timeout_ms`, up to the 60 s maximum.

## Can you run chemistry search via MCP on your own library?

Yes, mossflux serves any SMILES file as a database, and an agent searches your private library over MCP the same way it searches the public demo.

1. Prepare the file. The [CLI reference](https://mossflux.com/docs/cli) says mossflux reads `.smi`, `.smiles`, `.ism`, `.can`, TSV and ChEMBL chemreps files, gzip included.

 Two salt forms of one compound stay two rows.
2. Start the server with one `--db NAME=PATH` per library.

```sh
mossflux serve --db chembl=chembl_37_chemreps.txt.gz --db inhouse=inhouse.smi
```

3. Wait until `GET /ready` returns 200. A client that connects earlier gets no answer from the missing index.
4. Ask the agent to call `list_databases`. Each `NAME` you passed appears there, and searches then name the database they target.

ChEMBL 37 shows the memory cost. Loaded as a database named `chembl`, its 2,896,304 molecules take 33 to 41 s to build and the index takes 5.1 GB, about 2.5 KB of RAM per molecule.

Multiply 2.5 KB by your row count, and you've got a rough size for a smaller library. The deploy documentation also covers Docker, Compose and Kubernetes sizing.

There is no persisted index, no deletes and no sharding yet. Appending in memory through the segmented index changes no answer.

For speed, read the [benchmarks page](https://mossflux.com/docs/benchmarks). Its tables state machine, dataset, width, threshold, threads and trials. Its timings were taken before the 2026-10-03 hardening pass and were not re-measured on the new code, though the exactness gates pass on it.

## Frequently asked questions

### How do I start chemistry search via MCP with mossflux?

Start mossflux mcp for stdio, or point your client at the /mcp endpoint of mossflux serve for streamable HTTP. Then call similarity_search with a SMILES or substructure_search with a SMARTS. Add your own SMILES file with mossflux serve --db NAME=PATH.

### Does SMARTS substructure search work through MCP?

Yes. The substructure_search tool takes a SMARTS query and returns every molecule that contains the pattern.

### What happens when a substructure search times out?

The call returns an error, never a partial list. The server default deadline is 10 s and the maximum is 60 s, so narrow the SMARTS if you hit it.

### Do the fingerprints match RDKit, and are stereochemistry, salts and tautomers handled?

Chirality is parsed and ignored, and there is no salt or tautomer handling, so canonicalise your SMILES before loading them.

### Do I need an account or a hosted service?

You need neither. mossflux is free and open source under [MIT OR Apache-2.0](https://github.com/loruorg/mossflux), with no paid plan and no accounts. A [public demo](https://mossflux.com/) searches ChEMBL 37 if you want to try it first.

## What should you do next?

Today, add the public demo to your client and try chemistry search via MCP with one similarity search and one substructure search. In Claude Code, one command adds it:

```sh
claude mcp add --transport http mossflux https://api.mossflux.com/mcp
```

Send aspirin `CC(=O)Oc1ccccc1C(=O)O` as the similarity query and `S(=O)(=O)N` as the substructure query.

This week, install the CLI (or pull the Docker image) and serve your own SMILES file with `mossflux serve`.

```sh
cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

Next, give the agent a timeout and a hit cap, then read the [correctness page](https://mossflux.com/docs/correctness) to see what "exact" means. The [mossflux docs home](https://mossflux.com/docs/) holds the full reference.

mossflux returns what a brute-force scan returns, so [run it on your library](https://mossflux.com/#get) and compare its hits with your own scan.

## Sources

[API · mossflux docs](https://mossflux.com/docs/api) · [mossflux docs](https://mossflux.com/docs/) · [GitHub - cyanheads/pubchem-mcp-server: Search the PubChem…](https://github.com/cyanheads/pubchem-mcp-server) · [Deploy · mossflux docs](https://mossflux.com/docs/deploy) · [Transports - Model Context Protocol](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports) · [Design · mossflux docs](https://mossflux.com/docs/design) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks). Every other source is linked where the article cites it.
