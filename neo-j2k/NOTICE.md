<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Notices

neo-j2k is licensed under the BSD 2-Clause License. See `LICENSE`.

The header checks in `src/header.rs` and `src/header/`, JP2 box reader in `src/jp2.rs`, and
image metadata and reduce handling in `src/decoder.rs` and `src/geometry.rs` derive from
[OpenJPEG](https://github.com/uclouvain/openjpeg) through the C++ neo-j2k implementation.
The same is true of the tile walk and layout in `src/decode/` and `src/tile.rs`, the inverse
wavelets and colour transforms in `src/dwt.rs`, `src/dwt/`, `src/mct.rs` and `src/simd/`, packet and
code-block reading in `src/t1.rs`, `src/t1/`, `src/t2.rs` and `src/t2/`, and the encoder in
`src/encoder.rs` and `src/encoder/`.
See `LICENSE.OpenJPEG` for OpenJPEG's licence and copyright notices.
