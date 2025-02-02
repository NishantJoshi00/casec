# Stage 1: Base image for common dependencies
FROM rust:1.67 AS base

USER root

RUN \
  apt-get update \
  && apt-get install -y wget ca-certificates tzdata libpq-dev curl procps libuv1 libuv1-dev libssl-dev \
  && wget http://archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.0g-2ubuntu4_amd64.deb \
  && dpkg -i libssl1.1_1.1.0g-2ubuntu4_amd64.deb \
  && wget http://archive.ubuntu.com/ubuntu/pool/main/g/glibc/multiarch-support_2.27-3ubuntu1_amd64.deb \
  && dpkg -i multiarch-support_2.27-3ubuntu1_amd64.deb \
  && wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.15.3/cassandra-cpp-driver_2.15.3-1_amd64.deb \
  && wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.15.3/cassandra-cpp-driver-dev_2.15.3-1_amd64.deb \
  && dpkg -i cassandra-cpp-driver_2.15.3-1_amd64.deb cassandra-cpp-driver-dev_2.15.3-1_amd64.deb \
  && ln -s /usr/lib/x86_64-linux-gnu/libcassandra.so.2 /usr/lib/libcassandra.so

# Stage 2: Planner image
# FROM lukemathwalker/cargo-chef:latest-rust-1.78.0 AS chef
WORKDIR /app

COPY . .
# RUN cargo chef prepare --recipe-path recipe.json


# # Stage 3: Builder image
# FROM chef AS cooking
# WORKDIR /app

# COPY . .

# ARG CARGO_DIR
# ARG ACTIONS_RUNTIME_TOKEN
# ARG ACTIONS_CACHE_URL

# ENV CARGO_DIR=${CARGO_DIR}

# COPY $CARGO_DIR /usr/local/cargo

# RUN ls -l

# only do in local docker build
# RUN cargo install sccache

# ENV CARGO_INCREMENTAL=0
# ENV CARGO_NET_RETRY=2
# ENV RUSTUP_MAX_RETRIES=2
# ENV RUST_BACKTRACE="short"
# ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL="sparse"
# ENV SCCACHE_STATS=1
# ENV SCCACHE_GHA_ENABLED="on"
# ENV ACTIONS_CACHE_URL=${ACTIONS_CACHE_URL}
# ENV ACTIONS_RUNTIME_TOKEN=${ACTIONS_RUNTIME_TOKEN}
# ENV RUSTC_WRAPPER=/usr/local/cargo/sccache SCCACHE_DIR=~/.cache/sccache

# COPY --from=base / /


# COPY --from=chef /app/recipe.json recipe.json
# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#   --mount=type=cache,target=/usr/local/cargo/git \
#   --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
#   cargo chef cook --recipe-path recipe.json

# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#   --mount=type=cache,target=/usr/local/cargo/git \
RUN cargo build
# --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \

# RUN /usr/local/cargo/sccache --show-stats

# Stage 4: Final image
# FROM base AS final

ARG BIN_DIR=/local/bin
ARG BINARY=casec

RUN mkdir -p ${BIN_DIR}

RUN cp /app/target/debug/${BINARY} ${BIN_DIR}/${BINARY}

WORKDIR ${BIN_DIR}

CMD ./casec
