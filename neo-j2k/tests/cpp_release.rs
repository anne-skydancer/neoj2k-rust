// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The twin is pinned to one C++ release: the C++ checkout the oracle is built from reports the
//! crate's own version (`NJ2K_VERSION_*` of its `neoj2k.h`, through the shim). A checkout of another
//! release fails here first, before any differential test can mislead.

#[test]
fn the_cpp_checkout_is_the_crates_release() {
    let (major, minor, patch) = neo_j2k_oracle::cpp_version();
    assert_eq!(
        format!("{major}.{minor}.{patch}"),
        neo_j2k::version(),
        "NJ2K_CPP_SOURCE (or NJ2K_ORACLE_DIR) must be the neo-j2k checkout of the crate's release (README)"
    );
}
