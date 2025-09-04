# syntax=docker/dockerfile:1.4
####################################################################################################################################
#### COPIED FROM :: Alex Vincent :: https://github.com/f2calv/multi-arch-container-rust/blob/main/Dockerfile
####################################################################################################################################
FROM --platform=$BUILDPLATFORM rust AS base
WORKDIR /app
RUN apt-get update && apt-get upgrade -y
RUN rustup component add clippy
RUN rustup component add rustfmt

ARG TARGETPLATFORM
RUN \
if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
    apt-get install -y g++-x86-64-linux-gnu libc6-dev-amd64-cross ; \
    rustup target add x86_64-unknown-linux-gnu ; \
    rustup toolchain install stable-x86_64-unknown-linux-gnu ; \
elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    apt-get install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross ; \
    rustup target add aarch64-unknown-linux-gnu ; \
    rustup toolchain install stable-aarch64-unknown-linux-gnu ; \
elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then \
    apt-get install -y g++-arm-linux-gnueabihf libc6-dev-armhf-cross ; \
    rustup target add armv7-unknown-linux-gnueabihf ; \
    rustup toolchain install stable-armv7-unknown-linux-gnueabihf ; \
fi



FROM base AS dependencies
WORKDIR /app
#initialize an empty application & replace the dependencies file with our own (yes cargo chef can do this, but I feel this is simpler...)
RUN cargo init
COPY Cargo.toml /app
ARG TARGETPLATFORM
RUN \
if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
    TARGET=x86_64-unknown-linux-gnu ; \
elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    TARGET=aarch64-unknown-linux-gnu ; \
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc ; \
    export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc ; \
    export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ ; \
elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then \
    TARGET=armv7-unknown-linux-gnueabihf ; \
    export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc ; \
    export CC_armv7_unknown_Linux_gnueabihf=arm-linux-gnueabihf-gcc ; \
    export CXX_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++ ; \
fi \
&& cargo fetch --target $TARGET
#&& cargo build --release --target $TARGET
#https://github.com/f2calv/multi-arch-container-rust/issues/15



FROM dependencies AS source
COPY src src



FROM source AS build
ARG TARGETPLATFORM
RUN mkdir -p /build
RUN \
if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
    TARGET=x86_64-unknown-linux-gnu ; \
    echo 'TODO: need to complete and test building x86_64 FROM an arm platform??... ' ; \
elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    TARGET=aarch64-unknown-linux-gnu ; \
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc ; \
    export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc ; \
    export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ ; \
elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then \
    TARGET=armv7-unknown-linux-gnueabihf ; \
    export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc ; \
    export CC_armv7_unknown_Linux_gnueabihf=arm-linux-gnueabihf-gcc ; \
    export CXX_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++ ; \
fi \
&& cargo build --release --bin git-next-tag --target $TARGET && mv /app/target/$TARGET/release /build/release


#FROM bitnami/git:2.40.0
#FROM alpine/git:2.47.2
FROM cgr.dev/chainguard/git:latest

LABEL org.opencontainers.image.source = "https://github.com/joostvdg/git-next-tag-rust"
WORKDIR /work/

COPY --from=build /build/release/git-next-tag .
RUN ls -lath /work
RUN ls -lath /work/git-next-tag
RUN  ./git-next-tag --help

RUN git config --global safe.directory '*'
ENTRYPOINT ["./git-next-tag"]
CMD ["--help"]