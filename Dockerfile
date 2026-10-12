# syntax=docker/dockerfile:1
FROM rust:1-slim-bookworm AS build
# x86-64-v3 (AVX2) runs on any x86 server since 2015. Set TARGET_CPU=native
# for a host-only build, or x86-64-v4 for AVX-512.
ARG TARGET_CPU=""
WORKDIR /src
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    set -eu; \
    cpu="$TARGET_CPU"; \
    if [ -z "$cpu" ]; then \
      case "$(uname -m)" in x86_64) cpu=x86-64-v3 ;; aarch64) cpu=neoverse-n1 ;; *) cpu=generic ;; esac; \
    fi; \
    RUSTFLAGS="-C target-cpu=$cpu" cargo build --release --locked -p mossflux-cli; \
    cp target/release/mossflux /mossflux

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /mossflux /usr/local/bin/mossflux
EXPOSE 8080 50051
VOLUME /data
ENTRYPOINT ["/usr/local/bin/mossflux"]
CMD ["serve"]
