# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
#
# nj2k_rust_shim (-DNJ2K_BUILD_RUST_SHIM=ON): the test-only C interface that the Rust crate
# neo-j2k-oracle links for the twin's differential tests (README.md). It is built with the
# library's flags, never installed and never shipped. Its link manifest,
# nj2k-rust-shim-<config>.txt in the build folder, names every library the Rust link needs, in
# link order, so that the crate's build.rs needs no knowledge of the generator's folder layout.
add_library(nj2k_rust_shim STATIC shim.cpp layout_decoder.cpp layout_encoder.cpp)
target_include_directories(nj2k_rust_shim PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")
target_link_libraries(nj2k_rust_shim PUBLIC nj2k_test_common neoj2k_static)
nj2k_fp_strict(nj2k_rust_shim)
nj2k_simd(nj2k_rust_shim)
nj2k_program_exceptions(nj2k_rust_shim)
nj2k_warnings(nj2k_rust_shim)
nj2k_no_ipo(nj2k_rust_shim)
set_target_properties(nj2k_rust_shim PROPERTIES POSITION_INDEPENDENT_CODE ON)

# One line a library ("lib <path>") in link order, then the system libraries ("sys <name>") the
# C++ needs on this platform.
set(_nj2k_shim_sys "")
if(NOT WIN32)
  set(_nj2k_shim_sys "sys stdc++\nsys m\nsys pthread\n")
  if(CMAKE_DL_LIBS)
    string(APPEND _nj2k_shim_sys "sys ${CMAKE_DL_LIBS}\n")
  endif()
endif()
file(GENERATE OUTPUT "${PROJECT_BINARY_DIR}/nj2k-rust-shim-$<CONFIG>.txt"
     CONTENT "lib $<TARGET_FILE:nj2k_rust_shim>
lib $<TARGET_FILE:nj2k_test_common>
lib $<TARGET_FILE:nj2k_oracle>
lib $<TARGET_FILE:neoj2k_static>
${_nj2k_shim_sys}")
