// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The two halves of the size table (layout_decoder.cpp, layout_encoder.cpp).
#pragma once

#include <cstddef>

#include "nj2k_rust_shim.h"

namespace nj2k::shim {

extern const nj2k_shim_layout_t kDecoderLayout[];
extern const std::size_t kDecoderLayoutCount;
extern const nj2k_shim_layout_t kEncoderLayout[];
extern const std::size_t kEncoderLayoutCount;

}  // namespace nj2k::shim
