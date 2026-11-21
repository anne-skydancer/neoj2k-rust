<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Notices

neo-j2k is licensed under the BSD 2-Clause License. See `LICENSE`.

The header checks in `src/header.rs` and `src/header/`, JP2 box reader in `src/jp2.rs`, and
image metadata and reduce handling in `src/decoder.rs` and `src/geometry.rs` derive from
[OpenJPEG](https://github.com/uclouvain/openjpeg) through the C++ neo-j2k implementation.
The same is true of the inverse wavelets in `src/dwt.rs`, colour and sample operations in
`src/mct.rs`, and packet bit reader and tag trees in `src/t2.rs`.
See `LICENSE.OpenJPEG` for OpenJPEG's licence and copyright notices.
