#FROM rust:1.68-slim as builder
FROM rust:slim-bookworm as builder
WORKDIR /usr/src/git-next-tag
COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/git-next-tag/target \
    cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/git-next-tag/target \
    cargo build --release



FROM debian:bookworm-slim
ENTRYPOINT ["/usr/local/bin/git-next-tag"]
CMD ["--help"]
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/git-next-tag/target/release/git-next-tag /usr/local/bin/git-next-tag
