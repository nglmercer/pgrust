# syntax=docker/dockerfile:1

# Reproducible pgrust server image for a source checkout or fork.
#
# The runtime intentionally starts from the official PostgreSQL 18 image: the
# stock server-side binaries are removed, while the PostgreSQL 18 client tools
# (psql, pg_dump, pg_restore, pg_isready, pg_basebackup, ...) remain available.
# That matches the operational shape of the published pgrust image and gives
# backup/restore drills real PostgreSQL client binaries.

ARG RUST_IMAGE=rust:1.96.0-bookworm
# postgres:18-bookworm at PostgreSQL 18.6. Pin deliberately; upgrade this
# digest only together with a compatibility-matrix run.
ARG POSTGRES_BASE=postgres@sha256:f1ea2a1a4fb7b68d4836be1fdcb8a6466a89b072158bf9d97663600538c294c5

FROM ${RUST_IMAGE} AS build

ARG PGRUST_BUILD_ID=""
# dist matches the published artifact's fat-LTO profile. Use release for a
# quicker local evaluation build.
ARG PGRUST_CARGO_PROFILE=dist
ARG CARGO_BUILD_JOBS=""

ENV PGRUST_BUILD_ID=${PGRUST_BUILD_ID} \
    DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        git \
        libre2-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /src
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
# .cargo/config.toml wires the workspace's OpenSSL build script; without it
# the container build would not be the same build as the checkout.
COPY .cargo ./.cargo
COPY crates ./crates
COPY third_party ./third_party
COPY scripts ./scripts

RUN case "${PGRUST_CARGO_PROFILE}" in \
        release|dist) ;; \
        *) echo "PGRUST_CARGO_PROFILE must be release or dist" >&2; exit 2 ;; \
    esac \
    && jobs_args="" \
    && if [ -n "${CARGO_BUILD_JOBS}" ]; then jobs_args="--jobs ${CARGO_BUILD_JOBS}"; fi \
    && cargo build \
        --profile "${PGRUST_CARGO_PROFILE}" \
        --locked \
        -p main_main \
        -p initdb \
        --features main_main/full \
        ${jobs_args} \
    && mkdir -p /out/bin \
    && cp "target/${PGRUST_CARGO_PROFILE}/postgres" /out/bin/postgres \
    && cp "target/${PGRUST_CARGO_PROFILE}/initdb" /out/bin/initdb \
    && cp -a "target/${PGRUST_CARGO_PROFILE}/share" /out/share

FROM ${POSTGRES_BASE}

ARG PGRUST_BUILD_ID=""
ARG PGRUST_CARGO_PROFILE=dist
ARG PGRUST_SOURCE_URL=""

LABEL org.opencontainers.image.title="pgrust" \
      org.opencontainers.image.description="pgrust PostgreSQL-compatible server built from source" \
      org.opencontainers.image.revision="${PGRUST_BUILD_ID}" \
      org.opencontainers.image.source="${PGRUST_SOURCE_URL}" \
      dev.pgrust.cargo-profile="${PGRUST_CARGO_PROFILE}"

# Do not let PostgreSQL's server-side tools shadow or operate on a pgrust
# cluster. Client-side tools stay available for psql and logical backups.
RUN set -eu; \
    for bin in \
        postgres \
        initdb \
        pg_archivecleanup \
        pg_checksums \
        pg_controldata \
        pg_createsubscriber \
        pg_ctl \
        pg_resetwal \
        pg_rewind \
        pg_test_fsync \
        pg_test_timing \
        pg_upgrade \
        pg_waldump \
        pg_walsummary; \
    do rm -f "/usr/lib/postgresql/18/bin/${bin}"; done

COPY --from=build /out/bin/postgres /usr/local/bin/postgres
COPY --from=build /out/bin/initdb /usr/local/bin/initdb
COPY --from=build /out/share /usr/local/bin/share
# pgrust cannot use pg_ctl, so the image needs the pgrust-compatible port of
# the official entrypoint rather than the stock PostgreSQL entrypoint.
COPY docker/entrypoint.sh /usr/local/bin/docker-entrypoint.sh

RUN chmod 0755 /usr/local/bin/postgres /usr/local/bin/initdb /usr/local/bin/docker-entrypoint.sh \
    && ln -sfn /usr/local/bin/postgres /usr/local/bin/pgrust-postgres \
    && /usr/local/bin/postgres --version \
    && /usr/local/bin/initdb --version

ENV PGRUST_PGSHAREDIR=/usr/local/bin/share \
    RUST_MIN_STACK=33554432

# Inherited from the PostgreSQL base image:
#   ENTRYPOINT ["docker-entrypoint.sh"]
#   CMD ["postgres"]
#   PGDATA=/var/lib/postgresql/data
