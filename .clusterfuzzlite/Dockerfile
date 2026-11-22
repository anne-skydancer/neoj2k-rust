# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
#
# ClusterFuzzLite build image: OSS-Fuzz's Rust base builder (Clang, libFuzzer, rustup), the nightly
# pinned by date in nightly-toolchain and cargo-fuzz 0.13.2 (the versions fuzz/README.md names), and
# the repository. The base image is pinned by digest, so a moved tag cannot change what builds the
# sources; move the pin on purpose (the digest of base-builder-rust:v1 then).
FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:3428d0a90b1e7efbe2ba947bd1d0c9a70594986b4b6553804547f1fb24cfc92f
RUN apt-get update && apt-get install -y --no-install-recommends zip && rm -rf /var/lib/apt/lists/*
# the toolchain layers come before the sources, so a source change does not rebuild them
COPY nightly-toolchain /tmp/nightly-toolchain
RUN rustup toolchain install "$(cat /tmp/nightly-toolchain)" --profile minimal && \
    cargo install cargo-fuzz --version 0.13.2 --locked
COPY . $SRC/neoj2k-rust
WORKDIR $SRC/neoj2k-rust
COPY .clusterfuzzlite/build.sh $SRC/
