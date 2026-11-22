// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The encoder's half of the size table (layout_decoder.cpp has the decoder's): the types its arena
// holds. The tag tree types are named through EncBand's members, so this unit builds both before
// the ODR hotfix renames them (TagNode, TagTree) and after (EncTagNode, EncTagTree). encode.cpp's
// T1Job is not visible here; stage R5 adds it.
#include <cstdint>
#include <type_traits>

#include "../../../src/enc_tile.h"

#include "layout.h"

namespace nj2k::shim {

using namespace nj2k::detail;

namespace {
using EncTagTreeType = decltype(EncBand::incltree);
using EncTagNodeType = std::remove_pointer_t<decltype(EncTagTreeType::nodes)>;
}  // namespace

#define NJ2K_SHIM_LAYOUT(name, type) {name, sizeof(type), alignof(type)}

extern const nj2k_shim_layout_t kEncoderLayout[] = {
    NJ2K_SHIM_LAYOUT("EncComp", EncComp),
    NJ2K_SHIM_LAYOUT("EncRes", EncRes),
    NJ2K_SHIM_LAYOUT("EncCblk", EncCblk),
    NJ2K_SHIM_LAYOUT("EncPass", EncPass),
    NJ2K_SHIM_LAYOUT("EncLayer", EncLayer),
    NJ2K_SHIM_LAYOUT("EncTagNode", EncTagNodeType),
};
extern const std::size_t kEncoderLayoutCount = sizeof(kEncoderLayout) / sizeof(kEncoderLayout[0]);

}  // namespace nj2k::shim
