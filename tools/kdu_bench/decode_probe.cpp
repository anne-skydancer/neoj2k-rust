// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// Private, exact-build ABI adapter. No SDK code or viewer implementation is copied.
#include <windows.h>
#include <bcrypt.h>
#include <algorithm>
#include <array>
#include <chrono>
#include <cstdio>
#include <cstdint>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <stdexcept>
#include <string>
#include <vector>

using Bytes = std::vector<unsigned char>;
Bytes read_file(const wchar_t* path) {
    std::ifstream file(std::filesystem::path(path), std::ios::binary);
    if (!file) throw std::runtime_error("Cannot open input");
    Bytes result((std::istreambuf_iterator<char>(file)), {});
    if (result.empty()) throw std::runtime_error("Empty input");
    return result;
}
std::string sha256(const Bytes& data) {
    BCRYPT_ALG_HANDLE algorithm{}; BCRYPT_HASH_HANDLE hash{};
    if (BCryptOpenAlgorithmProvider(&algorithm, BCRYPT_SHA256_ALGORITHM, nullptr, 0) < 0)
        throw std::runtime_error("SHA256 provider failed");
    std::array<unsigned char, 32> digest{};
    bool ok = BCryptCreateHash(algorithm, &hash, nullptr, 0, nullptr, 0, 0) >= 0;
    if (ok) ok = BCryptHashData(hash, const_cast<unsigned char*>(data.data()), static_cast<ULONG>(data.size()), 0) >= 0;
    if (ok) ok = BCryptFinishHash(hash, digest.data(), static_cast<ULONG>(digest.size()), 0) >= 0;
    if (hash) BCryptDestroyHash(hash);
    BCryptCloseAlgorithmProvider(algorithm, 0);
    if (!ok) throw std::runtime_error("SHA256 failed");
    std::string result;
    for (auto byte : digest) { char text[3]; std::snprintf(text, sizeof(text), "%02x", byte); result += text; }
    return result;
}
template<class T> T symbol(HMODULE module, const char* name) {
    auto pointer = GetProcAddress(module, name);
    if (!pointer) throw std::runtime_error(std::string("Missing export: ") + name);
    return reinterpret_cast<T>(pointer);
}
struct Dims { int y, x, h, w; };
struct Source { void* vptr; const unsigned char* data; std::uint32_t size, pos; };
static_assert(sizeof(Source) == 24 && sizeof(Dims) == 16);

struct Codec {
    std::uintptr_t base;
    void (*create)(void**, void*, void*, void*);
    void (*destroy)(void**);
    void (*set_fast)(void**);
    void (*dims)(void**, int, Dims*, bool);
    int (*components)(void**, bool);
    int (*precision)(void**, int, bool, bool);
    bool (*is_signed)(void**, int, bool, bool);
    void (*tiles)(void**, Dims*);
    void* (*open_tile)(void**, void**, std::uint64_t, void*);
    void (*close_tile)(void**, void*, bool);
    void (*destroy_engine)(void*);
    void (*destroy_allocator)(void*);
    void* (*state_create)(void*, void*, unsigned char*, int, void**);
    bool (*state_process)(void*, float, bool);

    explicit Codec(HMODULE module) : base(reinterpret_cast<std::uintptr_t>(module)) {
        create = symbol<decltype(create)>(module, "?create@kdu_codestream@kdu_core@@QEAAXPEAVkdu_compressed_source@2@PEAVkdu_thread_env@2@PEAVkdu_membroker@2@@Z");
        destroy = symbol<decltype(destroy)>(module, "?destroy@kdu_codestream@kdu_core@@QEAAXXZ");
        set_fast = symbol<decltype(set_fast)>(module, "?set_fast@kdu_codestream@kdu_core@@QEAAXXZ");
        dims = symbol<decltype(dims)>(module, "?get_dims@kdu_codestream@kdu_core@@QEBAXHAEAUkdu_dims@2@_N@Z");
        components = symbol<decltype(components)>(module, "?get_num_components@kdu_codestream@kdu_core@@QEAAH_N@Z");
        precision = symbol<decltype(precision)>(module, "?get_bit_depth@kdu_codestream@kdu_core@@QEAAHH_N0@Z");
        is_signed = symbol<decltype(is_signed)>(module, "?get_signed@kdu_codestream@kdu_core@@QEAA_NH_N0@Z");
        tiles = symbol<decltype(tiles)>(module, "?get_valid_tiles@kdu_codestream@kdu_core@@QEAAXAEAUkdu_dims@2@@Z");
        open_tile = symbol<decltype(open_tile)>(module, "?open_tile@kdu_codestream@kdu_core@@QEAA?AVkdu_tile@2@Ukdu_coords@2@PEAVkdu_thread_env@2@@Z");
        close_tile = symbol<decltype(close_tile)>(module, "?close@kdu_tile@kdu_core@@QEAAXPEAVkdu_thread_env@2@_N@Z");
        destroy_engine = symbol<decltype(destroy_engine)>(module, "?destroy@kdu_push_ifc@kdu_core@@QEAAXXZ");
        destroy_allocator = symbol<decltype(destroy_allocator)>(module, "??1kdu_sample_allocator@kdu_core@@QEAA@XZ");
        state_create = reinterpret_cast<decltype(state_create)>(base + 0x1cd5040);
        state_process = reinterpret_cast<decltype(state_process)>(base + 0x1cd5820);
        // Kakadu-only startup entries, in their original CRT array order. They
        // initialize colour constants/dispatch, kernel tables, CPU globals and
        // block coding lookup tables. Viewer startup and its other CRT entries
        // remain disabled. See ABI.md for the address recovery evidence.
        for (std::uintptr_t rva : {0xf3560, 0xf3500, 0xf35f0, 0xf3670, 0xf3610,
                                  0xf3750, 0xf36f0, 0xf37c0, 0xf37d0, 0xf37f0,
                                  0xf3810, 0xf3830, 0xf3850, 0xf3870, 0xf3880})
            reinterpret_cast<void(*)()>(base + rva)();
    }

    // Fresh codec, original viewer tile engine and byte conversion, full teardown.
    // The caller owns and allocates the output outside this function.
    void decode(const Bytes& input, Bytes& storage, Dims& geometry, int& channels) {
        Source source{reinterpret_cast<void*>(base + 0x2dfba48), input.data(), static_cast<std::uint32_t>(input.size()), 0};
        void* stream = nullptr;
        create(&stream, &source, nullptr, nullptr);
        struct StreamGuard { Codec& codec; void** stream; ~StreamGuard() { if (*stream) codec.destroy(stream); } } guard{*this, &stream};
        set_fast(&stream); // The viewer's MODE_FAST path selects this explicitly.
        dims(&stream, 0, &geometry, false);
        channels = components(&stream, false);
        if (geometry.w < 1 || geometry.h < 1 || geometry.w > 8192 || geometry.h > 8192 || channels < 1 || channels > 4)
            throw std::runtime_error("Requires 1..4 components and dimensions in 1..8192");
        for (int c = 0; c < channels; ++c) {
            Dims component{}; dims(&stream, c, &component, false);
            if (std::memcmp(&component, &geometry, sizeof(Dims)) ||
                precision(&stream, c, false, false) != 8 || is_signed(&stream, c, false, false))
                throw std::runtime_error("Requires equal geometry, unsigned 8-bit components");
        }
        std::size_t count = std::size_t(geometry.w) * geometry.h * channels;
        if (storage.size() != count + 128) throw std::runtime_error("Output capacity mismatch");
        Dims tile_indices{}; tiles(&stream, &tile_indices);
        if (tile_indices.h != 1 || tile_indices.w != 1) throw std::runtime_error("Requires a single tile");
        void* tile = nullptr;
        std::uint64_t coords = std::uint32_t(tile_indices.y) | (std::uint64_t(std::uint32_t(tile_indices.x)) << 32);
        open_tile(&stream, &tile, coords, nullptr);
        alignas(64) unsigned char state[0x1d0]{};
        state_create(state, tile, storage.data() + 64, geometry.w * channels, &stream);
        struct StateGuard {
            Codec& codec; unsigned char* state; int channels;
            ~StateGuard() {
                for (int c = 0; c < channels; ++c) codec.destroy_engine(state + 0x180 + c * 8);
                codec.close_tile(reinterpret_cast<void**>(state + 0x1b8), nullptr, false);
                codec.destroy_allocator(state + 0x50);
            }
        } state_guard{*this, state, channels};
        if (!state_process(state, 0, false)) throw std::runtime_error("Tile decode incomplete");
    }
};

int wmain(int argc, wchar_t** argv) {
    SetErrorMode(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX);
    SetUnhandledExceptionFilter([](EXCEPTION_POINTERS* e)->LONG {
        std::fprintf(stderr, "Native fault: code=%08lx address=%p\n", e->ExceptionRecord->ExceptionCode, e->ExceptionRecord->ExceptionAddress);
        return EXCEPTION_EXECUTE_HANDLER;
    });
    if (argc != 6) { std::fprintf(stderr, "usage: kdu_decode_probe PRIVATE.dll INPUT.j2k EXPECTED.raw OUTPUT.raw ITERATIONS\n"); return 2; }
    try {
        // This digest pins every code address, layout and initializer below.
        // It is the original EXE with only DLL characteristic and entry RVA changed.
        if (sha256(read_file(argv[1])) != "dcd2a3726a3ad382681d55c14eec0120fbc5ba4c93279440cc7089f3ec4927da")
            throw std::runtime_error("Refusing an unrecognized binary; this adapter is exact-build only");
        int iterations = std::stoi(argv[5]);
        if (iterations < 0 || iterations > 100000) throw std::runtime_error("Invalid iteration count");
        HMODULE module = LoadLibraryExW(argv[1], nullptr, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS);
        if (!module) throw std::runtime_error("LoadLibraryEx failed: " + std::to_string(GetLastError()));
        Codec codec(module);
        auto input = read_file(argv[2]); auto expected = read_file(argv[3]);
        if (input.size() > 0x7fffffff) throw std::runtime_error("Input too large");
        Bytes storage(expected.size() + 128, 0xcd);
        Dims geometry{}; int channels = 0;
        auto verify = [&] {
            std::fill(storage.begin(), storage.end(), 0xcd);
            codec.decode(input, storage, geometry, channels);
            if (!std::all_of(storage.begin(), storage.begin()+64, [](auto b) { return b == 0xcd; }) ||
                !std::all_of(storage.end()-64, storage.end(), [](auto b) { return b == 0xcd; }))
                throw std::runtime_error("Output guard overwritten");
            if (!std::equal(expected.begin(), expected.end(), storage.begin()+64))
                throw std::runtime_error("Decoded pixels differ from reference; no timing accepted");
        };
        verify();
        std::vector<double> timings;
        if (iterations) {
            for (int i = 0; i < 16; ++i) codec.decode(input, storage, geometry, channels);
            for (int round = 0; round < 9; ++round) {
                auto start = std::chrono::steady_clock::now();
                for (int i = 0; i < iterations; ++i) codec.decode(input, storage, geometry, channels);
                double us = std::chrono::duration<double, std::micro>(std::chrono::steady_clock::now()-start).count()/iterations;
                timings.push_back(us);
            }
        }
        verify();
        std::ofstream output(std::filesystem::path(argv[4]), std::ios::binary);
        output.write(reinterpret_cast<char*>(storage.data()+64), expected.size()); output.close();
        if (!output) throw std::runtime_error("Cannot write output");
        int mmx = *symbol<int*>(module, "?kdu_mmx_level@kdu_core@@3HA");
        int bmi2 = *symbol<int*>(module, "?kdu_x86_bmi2_level@kdu_core@@3HA");
        std::printf("{\"width\":%d,\"height\":%d,\"components\":%d,\"bytes\":%zu,\"validated\":true,\"mmx_level\":%d,\"bmi2_level\":%d,\"iterations_per_round\":%d,\"round_us\":[", geometry.w, geometry.h, channels, expected.size(), mmx, bmi2, iterations);
        for (std::size_t i=0;i<timings.size();++i) std::printf("%s%.6f",i?",":"",timings[i]);
        std::printf("]}\n"); std::fflush(stdout);
        // Never run viewer shutdown handlers. Codec objects have already been
        // destroyed on every request; this process owns the mapped image.
        ExitProcess(0);
    } catch (const std::exception& error) { std::fprintf(stderr, "%s\n", error.what()); return 4; }
    catch (...) { std::fprintf(stderr, "Kakadu exception\n"); return 6; }
}
