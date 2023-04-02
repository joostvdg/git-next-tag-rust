FROM rust as builder
ARG APP_NAME="app"
ARG TARGET="aarch64-unknown-linux-musl"
RUN apt-get update
RUN apt-get install clang llvm -y
RUN rustup target add $TARGET
RUN mkdir /usr/src/$APP_NAME
WORKDIR /usr/src/$APP_NAME

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

ENV CC_aarch64_unknown_linux_musl=clang
ENV AR_aarch64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUNNER="qemu-aarch64 -L /usr/aarch64-linux-gnu"

COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

RUN mkdir -p /build
RUN cargo build --release --target=$TARGET && mv /usr/src/$APP_NAME/target/$TARGET/release /build/release

FROM alpine/git:v2.36.3
LABEL org.opencontainers.image.source = "https://github.com/joostvdg/git-next-tag-rust"
LABEL org.opencontainers.image.description ="git-next-tag is a tool to help you with the next version number of your project. It is based on the git history and the tags you have created. It is a simple tool that can be used in your CI/CD pipeline to determine the next version number of your project."
WORKDIR /work/

RUN adduser -D gitnexttag
RUN chown gitnexttag /work \
    && chmod "g+rwX" /work \
    && chown gitnexttag:root /work

COPY --from=builder --chown=gitnexttag:root /build/release/git-next-tag .

USER gitnexttag
RUN ls -lath /work
RUN ls -lath /work/git-next-tag
RUN  ./git-next-tag --help

RUN git config --global safe.directory '*'
ENTRYPOINT ["./git-next-tag"]
CMD ["--help"]