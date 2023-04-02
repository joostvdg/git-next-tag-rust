VERSION=0.5.3

build:
	cargo build --release

run:
	cargo run -- another test.txt -vv

run-verbose:
	cargo run -- another test.txt -vvv

test:
	cargo test

.PHONY: dpush-alpine-amd
dpush-alpine-amd:
	docker buildx build . -f docker/alpine/Dockerfile --platform linux/amd64 --tag ghcr.io/joostvdg/git-next-tag:$(VERSION)-alpine --provenance=false --sbom=false --push

.PHONY: dpush-alpine-arm
dpush-alpine-arm:
	docker buildx build . -f docker/alpine/Dockerfile --platform linux/arm64 --tag ghcr.io/joostvdg/git-next-tag:$(VERSION)-alpine --provenance=false --sbom=false --push

.PHONY: dpush-alpine
dpush-alpine:
	docker buildx build . \
		-f docker/alpine/Dockerfile \
		--platform linux/amd64,linux/arm64 \
		--tag ghcr.io/joostvdg/git-next-tag:$(VERSION)-alpine \
		--build-arg BUILDKIT_INLINE_BUILDINFO_ATTRS=1 \
		--provenance=false --sbom=false --push

.PHONY: dpush-amd
dpush-amd:
	docker buildx build . --platform linux/amd64 --tag ghcr.io/joostvdg/git-next-tag:$(VERSION)-amd --provenance=false --sbom=false --push

.PHONY: dpush-arm
dpush-arm:
	docker buildx build . --platform linux/arm64 --tag ghcr.io/joostvdg/git-next-tag:$(VERSION)-arm --provenance=false --sbom=false --push

.PHONY: dpush
dpush:
	docker buildx build . \
		--platform linux/amd64,linux/arm64 \
		--tag ghcr.io/joostvdg/git-next-tag:$(VERSION)-debian \
		--build-arg BUILDKIT_INLINE_BUILDINFO_ATTRS=1 \
		--provenance=false --sbom=false --push