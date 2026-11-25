---
description: Is there a mossflux arm64 docker image? No. The Docker Hub image is x86-64 only, so build from the Dockerfile on arm64. Learn how to verify it and size RAM.
faq:
  - answer: The loruorg/mossflux image on Docker Hub is x86-64 only, so on an arm64 host you build the image from the Dockerfile, following the build command on the deploy page. Building from source needs Rust 1.95 or later. Check the result with GET /ready and one similarity query before you rely on it.
    question: Is there a mossflux arm64 Docker image?
  - answer: QEMU can run the x86-64 mossflux image as linux/amd64 on arm64, which is enough for a quick API check. The container is emulated, so for a service you keep running, build the image from the Dockerfile on the arm64 host. Check the build with GET /ready and one similarity query.
    question: Can I run the x86-64 mossflux image on arm64 with QEMU?
  - answer: Build mossflux on the arm64 host that will run it. The deploy page says to build the image from the Dockerfile on arm64, so a build on the target host is the route it describes. Then call GET /ready and send one similarity query.
    question: Can I build mossflux on one arm64 machine and run it on another?
  - answer: The /ready endpoint returns 503 right after start because the server builds its index in memory at start and answers 503 until every database is loaded. Poll /ready until it returns 200, then send queries. The index is rebuilt on every start, so the wait repeats after each restart.
    question: Why does /ready return 503 right after the container starts?
  - answer: "Plan for about 2.5 KB of RAM per molecule, so multiply your library's molecule count by 2.5 KB to estimate the total. The index is rebuilt on every start. As a reference point, ChEMBL 37 has 2,896,304 molecules and a 5.1 GB index."
    question: How much RAM does mossflux need for my library?
image: /blog/mossflux-arm64-docker/featured-1f0e34ef85f9.webp
image_alt: "Illustration for Mossflux arm64 docker: Build it natively"
publishDate: "2026-11-25"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/mossflux-arm64-docker#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q624699\",\"@type\":\"Thing\",\"description\":\"free virtualization and emulation software\",\"name\":\"QEMU\",\"sameAs\":[\"https://en.wikipedia.org/wiki/QEMU\",\"https://www.wikidata.org/wiki/Q624699\",\"https://www.google.com/search?kgmid=/m/053qdz\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q388\",\"@type\":\"Thing\",\"description\":\"family of Unix-like operating systems\",\"name\":\"Linux\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Linux\",\"https://www.wikidata.org/wiki/Q388\",\"https://www.google.com/search?kgmid=/m/0fpzzp\"]}],\"datePublished\":\"2026-11-25\",\"description\":\"Is there a mossflux arm64 docker image? No. The Docker Hub image is x86-64 only, so build from the Dockerfile on arm64. Learn how to verify it and size RAM.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The loruorg/mossflux image on Docker Hub is x86-64 only, so on an arm64 host you build the image from the Dockerfile, following the build command on the deploy page. Building from source needs Rust 1.95 or later. Check the result with GET /ready and one similarity query before you rely on it.\"},\"name\":\"Is there a mossflux arm64 Docker image?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"QEMU can run the x86-64 mossflux image as linux/amd64 on arm64, which is enough for a quick API check. The container is emulated, so for a service you keep running, build the image from the Dockerfile on the arm64 host. Check the build with GET /ready and one similarity query.\"},\"name\":\"Can I run the x86-64 mossflux image on arm64 with QEMU?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Build mossflux on the arm64 host that will run it. The deploy page says to build the image from the Dockerfile on arm64, so a build on the target host is the route it describes. Then call GET /ready and send one similarity query.\"},\"name\":\"Can I build mossflux on one arm64 machine and run it on another?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The /ready endpoint returns 503 right after start because the server builds its index in memory at start and answers 503 until every database is loaded. Poll /ready until it returns 200, then send queries. The index is rebuilt on every start, so the wait repeats after each restart.\"},\"name\":\"Why does /ready return 503 right after the container starts?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Plan for about 2.5 KB of RAM per molecule, so multiply your library's molecule count by 2.5 KB to estimate the total. The index is rebuilt on every start. As a reference point, ChEMBL 37 has 2,896,304 molecules and a 5.1 GB index.\"},\"name\":\"How much RAM does mossflux need for my library?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q406283\",\"@type\":\"Thing\",\"description\":\"series of SoC and SiP processors from Apple\",\"name\":\"Apple silicon\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Apple_silicon\",\"https://www.wikidata.org/wiki/Q406283\",\"https://www.google.com/search?kgmid=/m/0gysszs\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q756100\",\"@type\":\"Thing\",\"description\":\"JavaScript runtime environment\",\"name\":\"Node.js\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Node.js\",\"https://www.wikidata.org/wiki/Q756100\",\"https://www.google.com/search?kgmid=/m/0bbxf89\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q364\",\"@type\":\"Organization\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q272629\",\"@type\":\"Thing\",\"description\":\"64-bit version of x86 architecture\",\"name\":\"X86-64\",\"sameAs\":[\"https://en.wikipedia.org/wiki/X86-64\",\"https://www.wikidata.org/wiki/Q272629\",\"https://www.google.com/search?kgmid=/m/01kdmy\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q66317867\",\"@type\":\"Thing\",\"description\":\"software\",\"name\":\"Docker Compose\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q66317867\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q202871\",\"@type\":\"Thing\",\"description\":\"software that enables one computer system (called the host) to behave like another computer system (called the guest)\",\"name\":\"Emulator\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Emulator\",\"https://www.wikidata.org/wiki/Q202871\",\"https://www.google.com/search?kgmid=/m/02p5w\"]}],\"name\":\"Mossflux arm64 docker: Build it natively\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/GApZgNnWHwguP1CHAUbPSlmFrLeJwgGf.webp\"},\"url\":\"https://mossflux.com/blog/mossflux-arm64-docker\"},{\"@id\":\"https://mossflux.com/blog/mossflux-arm64-docker#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q624699\"},{\"@id\":\"https://www.wikidata.org/wiki/Q388\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"Changelog · mossflux docs\",\"url\":\"https://mossflux.com/docs/changelog\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"Multi-platform builds | Docker Docs\",\"url\":\"https://docs.docker.com/build/building/multi-platform/\"},{\"@type\":\"CreativeWork\",\"name\":\"Codegen options - The rustc book\",\"url\":\"https://doc.rust-lang.org/rustc/codegen-options/index.html#target-cpu\"},{\"@type\":\"CreativeWork\",\"name\":\"OneUptime: Docker on Apple Silicon (M-series Macs)\",\"url\":\"https://oneuptime.com/blog/post/2026-01-16-docker-mac-apple-silicon/view\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"url\":\"https://turingpi.com/arm64-docker-compatibility/\"}],\"dateModified\":\"2026-11-25\",\"datePublished\":\"2026-11-25\",\"description\":\"Is there a mossflux arm64 docker image? No. The Docker Hub image is x86-64 only, so build from the Dockerfile on arm64. Learn how to verify it and size RAM.\",\"headline\":\"Mossflux arm64 docker: Build it natively\",\"image\":\"https://api.birchseek.com/m/GApZgNnWHwguP1CHAUbPSlmFrLeJwgGf.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/mossflux-arm64-docker#webpage\"},\"keywords\":[\"mossflux arm64 docker\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/mossflux-arm64-docker#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q406283\"},{\"@id\":\"https://www.wikidata.org/wiki/Q756100\"},{\"@id\":\"https://www.wikidata.org/wiki/Q364\"},{\"@id\":\"https://www.wikidata.org/wiki/Q272629\"},{\"@id\":\"https://www.wikidata.org/wiki/Q66317867\"},{\"@id\":\"https://www.wikidata.org/wiki/Q202871\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2282},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"(as a form of proximity search (metric space)) optimization problem of finding the point in a given set that is closest (or most similar) to a given point\",\"name\":\"Nearest neighbor search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Nearest_neighbor_search\",\"https://www.wikidata.org/wiki/Q608751\",\"https://www.google.com/search?kgmid=/m/025ytbk\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"cross-platform binary RPC protocol\",\"name\":\"GRPC\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GRPC\",\"https://www.wikidata.org/wiki/Q26356541\",\"https://www.google.com/search?kgmid=/g/11cp5mklv8\"]},{\"@type\":\"Thing\",\"description\":\"contextual queries\",\"name\":\"Semantic search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Semantic_search\",\"https://www.wikidata.org/wiki/Q1891170\",\"https://www.google.com/search?kgmid=/m/02r4_2d\"]},{\"@type\":\"Thing\",\"description\":\"database designed to store vector embeddings and perform similarity search in high-dimensional spaces\",\"name\":\"Vector database\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Vector_database\",\"https://www.wikidata.org/wiki/Q118396004\",\"https://www.google.com/search?kgmid=/g/11kph5bck7\"]},{\"@type\":\"Thing\",\"description\":\"group of atoms in a molecule which imparts specific chemical properties\",\"name\":\"Functional group\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Functional_group\",\"https://www.wikidata.org/wiki/Q170409\",\"https://www.google.com/search?kgmid=/m/02ydd\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - mossflux
  - arm64
  - docker
  - docker compose
  - kubernetes
  - qemu emulation
title: "Mossflux arm64 docker: Build it natively"
---

**The mossflux arm64 docker route is a local build: the `loruorg/mossflux` image on Docker Hub is x86-64 only (distroless, non-root), so on an arm64 host you build the image natively from the Dockerfile, as the [mossflux deploy docs](https://mossflux.com/docs/deploy) describe.

This mossflux arm64 docker guide is for anyone running mossflux on a small arm64 server or an Apple silicon laptop, where `docker pull loruorg/mossflux` isn't the whole answer. Start by asking Docker Hub which platforms the tag publishes:

```sh
docker buildx imagetools inspect loruorg/mossflux
```

You'll then be able to confirm the image platform, build a mossflux arm64 docker image, verify it with real SMILES and SMARTS queries, and size its memory for Compose or Kubernetes.

> **Key Takeaways**
> - Gate traffic on `GET /ready` returning 200, because a running container can still be loading its databases.
> - Budget about 2.5 KB of RAM per molecule, summed across every database you serve.
> - Publish port 8080 for REST and MCP, and port 50051 for gRPC.
> - Use emulation only for a throwaway API check, never for timing, capacity planning or production.

## Does a mossflux arm64 docker image exist?

There's no prebuilt mossflux arm64 docker image. On arm64, mossflux documents one route: build the image from the project's Dockerfile on an arm64 host, as the [deploy page](https://mossflux.com/docs/deploy) says.

Whether mossflux "supports arm64" splits into three questions with different answers: the image, correctness and speed.

The Docker Hub image is x86-64, so an arm64 host has no matching variant to pull. Release 0.1.0 is the initial release, as the [mossflux changelog](https://mossflux.com/docs/changelog) records.

On correctness, the [mossflux docs](https://mossflux.com/docs/) state that every answer is exactly what a brute-force scan returns. That rule belongs to the search design and carries no CPU architecture, so a similarity or substructure query must return the same rows as a brute-force scan on any host.

On speed, the [mossflux benchmarks page](https://mossflux.com/docs/benchmarks) publishes timings measured on x86 machines: AMD EPYC hardware, including an EPYC 9645 VM and an AVX2 `znver3` build.

Those timings also predate a code change. They were taken before the 2026-10-03 hardening pass and were not re-measured on the new code. The exactness gates do pass on it.

## How do you check which platform a Docker image and container use?

You check a Docker image's platform in three places, in order: the registry, the engine and the running container. Each answers a different question.

1. **Read the registry's platform list.** Ask the registry what a tag publishes:

```sh
docker buildx imagetools inspect loruorg/mossflux
```

Read the `os/arch` pairs in the manifest list, such as `linux/amd64` or `linux/arm64`. Entries that carry build attestations instead of an image are not platforms you can run, so they do not count as an arm64 variant.

2. **Check the engine.** Run `docker info` and read the `Architecture` line. It shows what the host runs, which decides whether an image runs natively or under emulation.

```sh
docker info
```

3. **Check the container.** Start the container and run `uname -m` inside it.

```sh
docker run --rm --platform linux/arm64 IMAGE uname -m
docker image inspect --format '{{.Os}}/{{.Architecture}}' loruorg/mossflux
```

The `--platform` flag asks for a specific variant.

4. **Separate a real gap from a failed lookup.** A real "no arm64 variant" means step 1 printed a platform list without `linux/arm64` in it. A failed lookup prints an error and no list: an authorisation failure, a network or DNS error, or "not found" from a misspelt name or tag. An error doesn't tell you anything about supported architectures, so fix the name, tag, login or connection and run step 1 again.

If the list for `loruorg/mossflux` shows `linux/amd64` and no `linux/arm64`, build the image from the Dockerfile on the arm64 host, following the Docker route in the [mossflux deploy docs](https://mossflux.com/docs/deploy). Then run step 3's `docker image inspect` on your build to check that it reports `linux/arm64`.

## How do you build a mossflux arm64 docker image?

You build the mossflux Docker image on arm64 by running the build from the repository's Dockerfile on an arm64 host, because the published `loruorg/mossflux` image is x86-64 only, as the [deploy page](https://mossflux.com/docs/deploy) says.

1. **Confirm the platform list.** Ask the registry what the tag publishes.

```sh
docker buildx imagetools inspect loruorg/mossflux
```

If the list shows only `linux/amd64`, the image has no arm64 variant. An error instead of a list is a failed lookup, so fix it first and run the command again.

2. **Clone the repository on the arm64 host.** Build on the machine that'll run the container. A build on an amd64 machine needs emulation or cross-compilation, which this route doesn't use.

3. **Run the documented build.** Copy the build command from the deploy page and keep the base images the Dockerfile names. The tag after `-t` is your choice.

```sh
git clone https://github.com/loruorg/mossflux && cd mossflux
docker build -t mossflux .
```

4. **Leave `--platform` off unless you need another target.** Docker's [multi-platform documentation](https://docs.docker.com/build/building/multi-platform/) defines a multi-platform build as "a single build invocation that targets multiple different operating system or CPU architecture combinations". A build on the arm64 host for that host needs no flag, so pass `--platform` only for a target other than the host's.

The [development page](https://mossflux.com/docs/development) covers the build and test steps.

### Source builds tie the binary to the build CPU

The CLI page documents a source install that compiles for the machine it runs on.

The [rustc book's entry on target-cpu](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-cpu) explains why: `native` can be passed "to use the processor of the host machine".

A CLI installed this way on one arm64 server is tied to CPUs like that server's. Build the CLI on each machine that will run it, instead of copying the binary to a server with a different CPU.

## Native build or QEMU emulation: which should you use on arm64?

Use a native arm64 build for anything you will time, size or keep running, and use QEMU or Rosetta emulation of the x86-64 image only for a throwaway functional check. A native build gives you an arm64 image of mossflux; emulation runs the x86-64 image on an arm64 host through an emulator.

| | Native arm64 build | Emulation (QEMU or Rosetta) |
| --- | --- | --- |
| What it needs | Dockerfile, arm64 host | x86-64 image, an emulator |
| Container platform | `linux/arm64` | `linux/amd64`, emulated |
| Quick API check | Yes | Yes |
| Timing or capacity numbers | Yes, your own run | No |
| Production | Yes | No |

Docker's [multi-platform documentation](https://docs.docker.com/build/building/multi-platform/) says QEMU emulation is slower than native builds, especially for compute-heavy tasks. That note is about building images and gives no multiplier for running a search service, and a ratio from another workload would not transfer. A mossflux query is compute-heavy work, so the note's direction applies to it.

On Apple silicon, a [OneUptime guide to Docker on M-series Macs](https://oneuptime.com/blog/post/2026-01-16-docker-mac-apple-silicon/view) draws the same split: an arm64 image runs natively, and an amd64 image runs "AMD64 via Rosetta" and is labelled emulated. Rosetta changes which emulator does the work; the container is still not native.

[mossflux's benchmarks page](https://mossflux.com/docs/benchmarks) records the machine, data and method of each run. A timing from your arm64 box, native or emulated, is a new measurement, separate from those results.

For a functional check, pull the x86-64 image, start it, and confirm `GET /health` answers and `GET /ready` turns 200. Then send one `POST /v1/similarity` with aspirin (`CC(=O)Oc1ccccc1C(=O)O`) as a quick look at the API, and throw the container away afterwards.

Keep emulation out of benchmarks, capacity plans and deployments. Any speed you read from an emulated container describes the emulator running the image, not mossflux on your CPU.

A native mossflux arm64 docker build suits a small arm64 server and a Compose or Kubernetes deployment. Emulation suits a first look at the REST API when you don't want to wait for a build.

## How do you verify a mossflux arm64 Docker container?

You verify a mossflux arm64 Docker container by calling four endpoints in a fixed order: `/health`, `/ready`, then one similarity search and one substructure search. The steps assume the container you built is running with port 8080 published and a database passed as `--db chembl=PATH`. Publish 50051 as well if you use gRPC.

1. **Call `/health`.** It shows whether the process is up and the REST port is reachable. It says nothing about the data.

```sh
curl -i http://localhost:8080/health
```

2. **Poll `/ready` until it returns 200.** The server builds its index in memory at start, and [mossflux's deploy docs](https://mossflux.com/docs/deploy) say `GET /ready` returns 503 until then. Wait for 200 before you send any search.

```sh
curl -i http://localhost:8080/ready
```

3. **Send a similarity search for aspirin.** The server fingerprints the SMILES itself, so the body holds only the structure and the search settings. Replace `chembl` with the database name you passed to `--db`.

```sh
curl -s -X POST http://localhost:8080/v1/similarity \
  -H 'Content-Type: application/json' \
  -d '{"smiles":"CC(=O)Oc1ccccc1C(=O)O","threshold":0.7,"limit":10,"db":"chembl"}'
```

4. **Send a substructure search for the sulfonamide.** Set `timeout_ms` so a slow match returns an error instead of hanging your shell.

```sh
curl -s -X POST http://localhost:8080/v1/substructure \
  -H 'Content-Type: application/json' \
  -d '{"smarts":"S(=O)(=O)N","limit":10,"timeout_ms":5000,"db":"chembl"}'
```

Both calls use fields documented in the [API reference](https://mossflux.com/docs/api), whose page "lists the endpoints, their fields and the limits."

The [correctness page](https://mossflux.com/docs/correctness) explains the tests behind the exactness rule, which holds on every platform. A substructure search that passes its deadline returns an error and never a partial list, so treat an error as a failed query rather than a short answer.

These four calls show that your mossflux arm64 docker container works. They aren't a benchmark and say nothing about speed on your arm64 CPU. To see the kind of answers to expect, run the same two queries on the [public demo](https://mossflux.com/#demo).

## How do you size and deploy mossflux on arm64 with Compose or Kubernetes?

You size and deploy mossflux on arm64 by budgeting RAM for an index rebuilt on every start, publishing both ports, and gating traffic on `/ready`. The [mossflux deploy page](https://mossflux.com/docs/deploy) covers Docker, Compose and Kubernetes, and these checks apply on any platform.

1. **Budget memory first.** mossflux builds the index in memory at start, does not save it to disk, and rebuilds it on every start. Plan about 2.5 KB of RAM per molecule. The worked example is ChEMBL 37 with 2,896,304 molecules: the build takes 33 to 41 s and the index is 5.1 GB.

2. **Scale the figure to your library.** Multiply your molecule count by 2.5 KB: a library of 1,000,000 molecules needs about 2.5 GB.

3. **Publish both ports.** If you map only 8080, gRPC clients can't connect.

4. **Build the image locally on arm64.** Run the build on the arm64 host and tag it with a name of your choice:

```sh
docker build -t mossflux:local .
```

5. **Point the manifest at your image.** The repository's `deploy/` directory holds a Docker Compose file and Kubernetes manifests. Replace the image name in whichever you use with your local tag. On Kubernetes, a multi-node cluster needs the image on every arm64 node that can run the pod, or in a registry those nodes can pull from.

6. **Wire the probes.** Use `/health` for liveness and `/ready` for readiness, so the pod receives traffic only after every database has loaded.

7. **Check success.** Once `/ready` returns 200, the service accepts searches. Confirm it from the host:

```sh
curl -i http://localhost:8080/ready
```

A 503 means the index is still building, so wait and call again.

Each request returns at most 10,000 hits. The server runs 4 searches at once by default, and further searches wait.

## Frequently asked questions

### Is there a mossflux arm64 Docker image?

The loruorg/mossflux image on Docker Hub is x86-64 only, so on an arm64 host you build the image from the Dockerfile, following the build command on the deploy page. Check the result with GET /ready and one similarity query before you rely on it.

### Can I run the x86-64 mossflux image on arm64 with QEMU?

Emulation can start the x86-64 image as linux/amd64, which is enough for a quick API check. The container is not native arm64, so build from the Dockerfile for a service you keep running.

### Can I build mossflux on one arm64 machine and run it on another?

Build the image on the arm64 host that will run it. Then call GET /ready and send one similarity query.

### Why does /ready return 503 right after the container starts?

Poll /ready until it returns 200, then send queries.

### How much RAM does mossflux need for my library?

Plan about 2.5 KB of RAM per molecule, and the index is rebuilt on every start. ChEMBL 37 has 2,896,304 molecules and a 5.1 GB index.

## What to do next

Today, run `docker buildx imagetools inspect loruorg/mossflux` and read the platform list. For a mossflux arm64 docker setup, that list tells you whether you need to build locally.

This week, build the image on your arm64 host, then call `/health`, `/ready`, one `POST /v1/similarity` with aspirin `CC(=O)Oc1ccccc1C(=O)O` and one `POST /v1/substructure` with `S(=O)(=O)N`. Multiply your molecule count by the per-molecule RAM figure on the [deploy page](https://mossflux.com/docs/deploy) to size memory.

Later, put the server in Compose or Kubernetes with `/ready` as the readiness probe and ports 8080 and 50051 published. Then time your own queries on your own library, recording conditions the way the [benchmarks page](https://mossflux.com/docs/benchmarks) gives the machine, data and method of each published run.

[Run it on your library](https://mossflux.com/#get) and compare its rows with a brute-force scan of your own data.

## Sources

[Deploy · mossflux docs](https://mossflux.com/docs/deploy) · [Changelog · mossflux docs](https://mossflux.com/docs/changelog) · [mossflux docs](https://mossflux.com/docs/) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks) · [turingpi.com](https://turingpi.com/arm64-docker-compatibility/) · [Multi-platform builds | Docker Docs](https://docs.docker.com/build/building/multi-platform/) · [Codegen options - The rustc book](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-cpu) · [API · mossflux docs](https://mossflux.com/docs/api). Every other source is linked where the article cites it.
