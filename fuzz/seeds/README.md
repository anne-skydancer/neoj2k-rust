<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Fuzz seeds

- `jp2/`: JP2 files for `header_decode` and `differential` (a three-channel RGB file and a palette file).
- `api_sequence/`: the call sequences of the `api_sequence` target (failure and retry, option changes).
- `roundtrip/`: coding parameters and samples for `roundtrip` (twelve parameter bytes, then samples): every progression, mode switch, 1 to 5 components, 1 to 16 bits, signed samples, 1 to 6 resolutions, code-block sizes, layers, SOP, EPH and the colour transform. Copied from the C++ repository (`fuzz/seeds/roundtrip`), whose round-trip target reads the same format.
- `encode/`: the option bytes and samples of `encode_options`, copied from the C++ repository (`fuzz/seeds/encode`).
- `../../corpus/generated` seeds `header_decode` and `differential`.
- `t1/`: code-blocks for `t1` (a header of six bytes, two bytes a segment, then the bytes; `../src/t1.rs` has the format), a few small ones of each code-block style taken from the C++ repository's committed vectors `tests/vectors/t1`. Copied from the C++ repository (`fuzz/seeds/t1`), whose `fuzz_t1` reads the same format.
