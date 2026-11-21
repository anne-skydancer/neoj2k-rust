// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The decoder's half of the size table: the types its arenas hold (Arena::make_array, make,
// ArenaList; WorkArena::make_array, raw_array, grow) and the Tier-1 scratch that kept_bytes()
// counts, as the twin's ledger charges them (neo-j2k/src/mem/layout.rs). The decoder's and
// the encoder's headers are kept in two translation units: both define nj2k::detail::TagNode and
// TagTree on origin/main before the ODR hotfix (plan finding C1).
//
// Types defined inside a .cpp file (decode.cpp's TileWalk, t2.cpp's PiResolution and PiComp) are
// not visible here; the stage that transcribes their unit adds them (README.md).
#include <cstdint>

#include <neoj2k/neoj2k.h>

#include "../../../src/decode.h"
#include "../../../src/dwt.h"
#include "../../../src/header.h"
#include "../../../src/jp2.h"
#include "../../../src/t1.h"
#include "../../../src/tile.h"

#include "layout.h"

namespace nj2k::shim {

using namespace nj2k::detail;

#define NJ2K_SHIM_LAYOUT(name, type) {name, sizeof(type), alignof(type)}

extern const nj2k_shim_layout_t kDecoderLayout[] = {
    NJ2K_SHIM_LAYOUT("u8", std::uint8_t),
    NJ2K_SHIM_LAYOUT("i16", std::int16_t),
    NJ2K_SHIM_LAYOUT("u32", std::uint32_t),
    NJ2K_SHIM_LAYOUT("u64", std::uint64_t),
    NJ2K_SHIM_LAYOUT("bool", bool),
    NJ2K_SHIM_LAYOUT("ImageComp", nj2k_image_comp_t),
    NJ2K_SHIM_LAYOUT("SizComponent", SizComponent),
    NJ2K_SHIM_LAYOUT("CodingStyle", CodingStyle),
    NJ2K_SHIM_LAYOUT("Quantization", Quantization),
    NJ2K_SHIM_LAYOUT("PocEntry", PocEntry),
    NJ2K_SHIM_LAYOUT("TlmEntry", TlmEntry),
    NJ2K_SHIM_LAYOUT("PackedChunk", PackedChunk),
    NJ2K_SHIM_LAYOUT("TileOverrides", TileOverrides),
    NJ2K_SHIM_LAYOUT("TileDirectoryPage", TileDirectory::Page),
    NJ2K_SHIM_LAYOUT("MainHeader", MainHeader),
    NJ2K_SHIM_LAYOUT("TileComponentParams", TileComponentParams),
    NJ2K_SHIM_LAYOUT("Jp2Header", Jp2Header),
    NJ2K_SHIM_LAYOUT("Jp2CmapEntry", Jp2CmapEntry),
    NJ2K_SHIM_LAYOUT("Jp2CdefEntry", Jp2CdefEntry),
    NJ2K_SHIM_LAYOUT("OutComp", OutComp),
    NJ2K_SHIM_LAYOUT("ResRect", ResRect),
    NJ2K_SHIM_LAYOUT("TileComp", TileComp),
    NJ2K_SHIM_LAYOUT("Resolution", Resolution),
    NJ2K_SHIM_LAYOUT("Precinct", Precinct),
    NJ2K_SHIM_LAYOUT("CodeBlock", CodeBlock),
    NJ2K_SHIM_LAYOUT("TagNode", TagNode),
    NJ2K_SHIM_LAYOUT("Segment", Segment),
    NJ2K_SHIM_LAYOUT("Chunk", Chunk),
    NJ2K_SHIM_LAYOUT("T1", T1),
};
extern const std::size_t kDecoderLayoutCount = sizeof(kDecoderLayout) / sizeof(kDecoderLayout[0]);

}  // namespace nj2k::shim
