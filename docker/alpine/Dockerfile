FROM rust AS base

# amd64-specific stage
FROM base AS build-amd64
WORKDIR /usr/src/git-next-tag
ENV TARGET=x86_64-unknown-linux-musl
RUN apt update && apt install -y g++-mingw-w64-x86-64 musl-tools
RUN rustup target add x86_64-unknown-linux-musl

COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY ./src ./src
RUN mkdir -p /build
RUN cargo build --release --target=$TARGET && mv /usr/src/git-next-tag/target/$TARGET/release /build/release

# arm64-specific stage
FROM base AS build-arm64
ARG APP_NAME="app"
ARG TARGET="aarch64-unknown-linux-musl"
RUN apt-get update && apt-get install clang llvm -y
RUN rustup target add $TARGET
RUN mkdir /usr/src/$APP_NAME
WORKDIR /usr/src/$APP_NAME

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

ENV CC_aarch64_unknown_linux_musl=clang
ENV AR_aarch64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUNNER="qemu-aarch64 -L /usr/aarch64-linux-gnu"

COPY Cargo.toml ./
COPY ./src ./src

RUN mkdir -p /build
RUN cargo build --release --target=$TARGET && mv /usr/src/$APP_NAME/target/$TARGET/release /build/release


# common steps
FROM build-${TARGETARCH} AS build
RUN echo "This stage is used on all architectures"
RUN ls -lath /build/release

# common steps
FROM alpine/git:2.49.1
LABEL org.opencontainers.image.source="https://github.com/joostvdg/git-next-tag-rust"
LABEL org.opencontainers.image.description="git-next-tag is a tool to help you with the next version number of your project. It is based on the git history and the tags you have created. It is a simple tool that can be used in your CI/CD pipeline to determine the next version number of your project."
LABEL org.opencontainers.image.authors="Joost van der Griendt <joostvdg@gmail.com>"
WORKDIR /work/

RUN addgroup -S appgroup && adduser -D -u 1000 gitnexttag -G appgroup -h /work

COPY --from=build --chown=gitnexttag:appgroup /build/release/git-next-tag .

USER gitnexttag
RUN ls -lath /work
RUN ls -lath /work/git-next-tag
RUN  ./git-next-tag --help

RUN git config --global safe.directory '*'
ENTRYPOINT ["./git-next-tag"]
CMD ["--help"]
