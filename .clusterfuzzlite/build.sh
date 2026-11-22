#!/bin/bash -eu
# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
#
# ClusterFuzzLite build of the targets that need no C++: header_decode and roundtrip (the others
# have required-features = ["oracle"], so a plain build skips them). The targets that compare with
# the C++ oracle (differential, encode_options, api_sequence) run in the nightly workflow, which
# has the C++ checkout. The sanitizer arrives in SANITIZER: AddressSanitizer for fuzzing, none for
# coverage (the image sets the coverage RUSTFLAGS). The build is optimised, with debug assertions,
# so an arithmetic overflow panics. Seeds: the generated corpus and the JP2 seeds for header_decode,
# fuzz/seeds/roundtrip for roundtrip; dictionary: fuzz/j2k.dict.
cd "$SRC/neoj2k-rust"
nightly="$(cat nightly-toolchain)"
case "${SANITIZER:-address}" in
  coverage) sanitizer=none ;;
  address) sanitizer=address ;;
  *) echo "unsupported sanitizer: $SANITIZER" >&2; exit 1 ;;
esac
cargo +"$nightly" fuzz build --fuzz-dir fuzz --sanitizer "$sanitizer" -O --debug-assertions
target_dir=fuzz/target/x86_64-unknown-linux-gnu/release
for t in header_decode roundtrip; do
  cp "$target_dir/$t" "$OUT/"
  cp fuzz/j2k.dict "$OUT/$t.dict"
done
zip -j -q "$OUT/header_decode_seed_corpus.zip" corpus/generated/*.j2k fuzz/seeds/jp2/*.jp2
zip -j -q "$OUT/roundtrip_seed_corpus.zip" fuzz/seeds/roundtrip/*.bin
