FROM rust:slim-bookworm as builder
WORKDIR /usr/src/git-next-tag

ENV TARGET=x86_64-unknown-linux-musl
RUN rustup target add x86_64-unknown-linux-musl

RUN apt update && apt install -y g++-mingw-w64-x86-64

RUN rustup target add x86_64-pc-windows-gnu
RUN rustup toolchain install stable-x86_64-pc-windows-gnu

COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/git-next-tag/target \
    cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY ./src ./src
RUN mkdir -p /build
RUN cargo build --release --target=$TARGET && mv /usr/src/git-next-tag/target/$TARGET/release /build/release


FROM alpine/git:v2.36.3
LABEL org.opencontainers.image.source = "https://github.com/joostvdg/git-next-tag-rust"
LABEL org.opencontainers.image.description ="git-next-tag is a tool to help you with the next version number of your project. It is based on the git history and the tags you have created. It is a simple tool that can be used in your CI/CD pipeline to determine the next version number of your project."
WORKDIR /work/

RUN adduser -D gitnexttag
RUN chown gitnexttag /work \
    && chmod "g+rwX" /work \
    && chown gitnexttag:root /work

COPY --from=builder --chown=gitnexttag:root /usr/local/cargo/bin/git-next-tag /work/application

USER gitnexttag
RUN git config --global safe.directory '*'
ENTRYPOINT ["./application"]
CMD ["--help"]