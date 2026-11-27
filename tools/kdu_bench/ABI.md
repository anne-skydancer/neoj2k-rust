# Exact-build ABI recovery

The address map below applies only to the SHA-256-pinned Firestorm 81788 EXE in
README.md. Addresses are RVAs, added to the actual mapped base. No viewer or SDK
implementation has been copied into this repository; the adapter calls the
routines in the user's supplied binary.

## Decode path

MSVC RTTI `.?AVLLImageJ2CKDU@@` identifies TypeDescriptor `0x34ec1a8` and complete
object locator `0x2f97108`. The vtable at `0x2dfbbc8` has `decodeImpl` at slot 2,
RVA `0x1cd37d0`. Its call sequence establishes:

| Item | RVA / evidence |
| --- | --- |
| Tile decode-state allocation | `0x1cd3bc0`: size `0x1d0` |
| State constructor | `0x1cd5040`, called from `0x1cd3bed` |
| State process SEH wrapper | `0x1cd5800`, called from `0x1cd3c44` |
| Actual state process | `0x1cd5820`, called by the SEH wrapper |
| Pixel buffer in state | offset `0x1c0`, advanced by row gap at `0x1c8` |
| Component engine handles | offset `0x180`, stride 8, one per component |
| Tile handle / sample allocator | offsets `0x1b8` / `0x50` |

The process routine pulls component lines, calls the original inverse colour
transform, then invokes the viewer's byte-transfer routine (`0x1cd4c50`). The
constructor accepts `(this, tile-handle-by-value, output, row-gap, codestream*)`.
Calls in the original wrapper establish the Windows x64 register/stack ABI.
The handle returned from exported `open_tile` uses a hidden return-buffer argument.
Kakadu coordinates are ordered y,x, so dimensions are `{y,x,height,width}`.

MSVC RTTI `.?AVLLKDUMemSource@@` identifies TypeDescriptor `0x34ec180`, complete
object locator `0x2f97358` and vtable `0x2dfba48`. Its read routine (`0x1cd2230`)
loads the input pointer at offset 8, size at 16 and position at 20. Its deleting
destructor confirms the object size of 24. The adapter supplies these fields and
uses the existing vtable without invoking that destructor.

The wrapper's successful teardown sequence is visible at `0x1cd3c60` onward:
engine destroy, tile close, allocator destroy, then state storage release. The
adapter repeats those operations with stack-owned state, followed by exported
codestream destruction. No retained decode state spans timed requests.

## Kakadu initialization

With the viewer entry point disabled, the first RGB attempt decoded lines but
faulted through a null colour-conversion dispatch pointer. Calling only the
colour initializer removed the fault, but returned incorrect pixels, even for
one-pixel grayscale. Initializing the block-coding tables resolved that mismatch.

The original CRT array contains Kakadu startup entries at raw file offsets
`0x288d980` through `0x288da08`, mixed with exit registrations. The adapter invokes
only the following examined codec entries, in their original relative order:

| RVAs | Role |
| --- | --- |
| `0xf3560`, `0xf3500` | Fixed-point colour constants |
| `0xf35f0` | Colour dispatch initialization; calls `0x220dbb0` |
| `0xf3670`, `0xf3610`, `0xf3750`, `0xf36f0` | Additional fixed-point colour constants |
| `0xf37c0` | Kernel/energy lookup initialization |
| `0xf37d0`, `0xf37f0`, `0xf3810` | CPU capability globals, using original CPUID/XGETBV detection |
| `0xf3830`, `0xf3850`, `0xf3870`, `0xf3880` | Block-coding lookup initialization |

Exit-registration entries `0xf35d0`, `0xf35e0` and `0xf36e0` are excluded. Other
application CRT initializers are not invoked. This subset has passed the reported
lossless tests; it does not establish correctness for every Kakadu feature.

## Validation

The safe Rust reference decodes the identical codestream and writes full-precision
8-bit interleaved pixels. It does not clamp values or flip/transpose output to
obtain a match. The native adapter compares every output byte before and after
repeated fresh-codec calls and checks 64-byte guards on both ends of the buffer.
Default cases cover 1x1 grayscale/RGB, 1x64 grayscale, 3x5 grayscale, 4x4 RGB,
7x9 RGB, multilayer 16x16 RGB, 32x32 RGBA, multilayer 48x48 grayscale and 2048x64
RGBA. All ten matched exactly on Windows x64 on 2026-11-25.

This is evidence for retrieving correct pixels from this binary, not a claim
that an unofficial ABI is portable or that the SDK is unnecessary for supported
application integration.
