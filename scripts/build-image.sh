#!/usr/bin/env bash
# Build a fork-specific pgrust Docker image from this checkout.
#
# Environment knobs:
#   IMAGE_NAME             image tag to write (default: pgrust-fork:local)
#   PGRUST_CARGO_PROFILE   dist or release (default: dist; release is faster)
#   CARGO_BUILD_JOBS       optional cargo parallelism limit
#   POSTGRES_BASE          optional runtime base override, preferably a digest
#   RUST_IMAGE             optional Rust build-image override
#   EXTRA_DOCKER_BUILD_ARGS optional extra arguments for docker build, split by the shell

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
IMAGE_NAME="${IMAGE_NAME:-pgrust-fork:local}"
PGRUST_CARGO_PROFILE="${PGRUST_CARGO_PROFILE:-dist}"

case "$PGRUST_CARGO_PROFILE" in
  release|dist) ;;
  *)
    echo "PGRUST_CARGO_PROFILE must be release or dist" >&2
    exit 2
    ;;
esac

if git -C "$ROOT" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  BUILD_ID="$(git -C "$ROOT" rev-parse --short=12 HEAD)"
  if [ -n "$(git -C "$ROOT" status --porcelain)" ]; then
    BUILD_ID="${BUILD_ID}-dirty"
  fi
  SOURCE_URL="$(git -C "$ROOT" remote get-url origin 2>/dev/null || true)"
else
  BUILD_ID="source-tree"
  SOURCE_URL=""
fi

echo "building $IMAGE_NAME"
echo "  profile:  $PGRUST_CARGO_PROFILE"
echo "  build id: $BUILD_ID"

# EXTRA_DOCKER_BUILD_ARGS is intentionally word-split so callers can pass normal
# docker CLI arguments, for example: EXTRA_DOCKER_BUILD_ARGS='--platform linux/amd64'
# shellcheck disable=SC2086
docker build \
  --file "$ROOT/Dockerfile" \
  --tag "$IMAGE_NAME" \
  --build-arg "PGRUST_BUILD_ID=$BUILD_ID" \
  --build-arg "PGRUST_SOURCE_URL=$SOURCE_URL" \
  --build-arg "PGRUST_CARGO_PROFILE=$PGRUST_CARGO_PROFILE" \
  --build-arg "CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-}" \
  --build-arg "POSTGRES_BASE=${POSTGRES_BASE:-postgres@sha256:f1ea2a1a4fb7b68d4836be1fdcb8a6466a89b072158bf9d97663600538c294c5}" \
  --build-arg "RUST_IMAGE=${RUST_IMAGE:-rust:1.96.0-bookworm}" \
  ${EXTRA_DOCKER_BUILD_ARGS:-} \
  "$ROOT"

echo "built $IMAGE_NAME"
docker image inspect "$IMAGE_NAME" --format '  image id: {{.Id}}'
