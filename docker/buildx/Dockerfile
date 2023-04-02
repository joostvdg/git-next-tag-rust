# syntax=docker/dockerfile:1

FROM alpine AS base

# arm64-specific stage
FROM base AS build-arm64
RUN echo "This stage is used on arm64"

# amd64-specific stage
FROM base AS build-amd64
RUN echo "This stage is used on amd64 (x86)"

# common steps
FROM build-${TARGETARCH} AS build
RUN echo "This stage is used on all architectures"