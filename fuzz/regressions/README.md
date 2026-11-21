<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Fuzz regression inputs

An input a fuzzer found a defect with lands here once the code under test is fixed, named
`<target>-<finding>.bin` (for example `decode-ppm-eph-cut.bin`). CTest replays this folder through
every fuzz target on every platform (`fuzz_replay.*`, with the seeds), so a fixed defect stays fixed.

An input whose defect is still open, in the oracle or in neo-j2k, does not belong here when the
target would only skip it. It goes to `tests/known-defects` with a check that it still reproduces (the
check fails once the defect is gone) and an entry in `doc/oracle.md`. An input the target still runs,
checking less of it because of an open oracle defect, stays here, and its known-defect check replays
it with the guard off.

| Input | Finding |
|---|---|
| `roundtrip-guard-bit-overflow.bin` | ClusterFuzzLite on the stage-0 pull request: a lossless round trip of the oracle came back with other samples. OpenJPEG's encoder wrote a stream that cannot hold the image (OD-6, too few guard bits), and the target encoded and decoded such inputs without comparing them. Since the pin moved to the fork's hotfix head (`841b2de4`) the encoder refuses this image, and the target compares every stream the encoder writes. |
| `roundtrip-mqc-termall-16bit.bin` | ClusterFuzzLite with AddressSanitizer on the stage-0 pull request, the round-trip target: a one-byte heap write past a code-block buffer of the oracle's MQ encoder (OD-4: TERMALL on three 16-bit components). It was a known defect while the target skipped TERMALL above 8 bits. The pin (`e64db843`) sizes the buffer for the terminated passes, so the target skips nothing: the code-blocks of this input are coded with AddressSanitizer watching the buffers (the pin before overflows here, this one does not), and the encode then fails in Tier-2, as OpenJPEG refuses such an image ("only 12 bytes remaining in output buffer"). |
| `header-jp2-pclr-values-before-size.bin` | Stage 1b, the header target: a JP2 palette box declaring 400 entries of 255 columns in 774 bytes; the reader allocated the 408 KB of values before checking the box holds them. |
| `header-jp2-image-box-before-ftyp.bin` | Stage 1b's review round, ClusterFuzzLite with UBSan (run 44526231005), the header target comparing with the oracle: a JP2 file with a bits-per-component box where the file type box belongs. OpenJPEG reads it (its skip of the misplaced box sets every state bit, DP-11), neo-j2k refuses it, as the standard has it; the target did not know the departure. It now does, and replays this input. |
| `decode-header-side-above-limit.bin` | Stage 1c, ClusterFuzzLite with UBSan on the pull request (run 44557282350), the decode target: an out-of-memory (3,196 MB against 2,560 MB). Its SIZ declares 1,073,741,825 x 1 pixels in three components (options: reduce 5, viewer mode); `nj2k_read_header_memory()` reads the header, which is right (the decoded side is checked when decoding starts), but the target sized its 8-bit planes from it before any limit was checked. It now sizes them only once `nj2k_get_decoded_dimensions()` accepts the components, and checks that the decode refuses what that call refuses, with the same status. |
| `decode-poc-before-cod.bin` | Stage 1c's review round: `poc_lrcp_rlcp.j2k` with its POC moved into the main header before the COD. OpenJPEG bounds the POC's layer end by the layers known when it reads it (none yet) and decodes a flat image; neo-j2k decodes the file's image (DP-12). The decode target's departure check did not know the case and would have stopped on it; it now does. |
| `encode-size-cap-lossless.bin` | ClusterFuzzLite with MemorySanitizer on pull request #10 (run 44835414266), the encode target: "lossless: component 0 sample 0 is 200, was 206 (64x64)". A 64 x 64 image in three components, 5/3, no layers asked and a 5,000-byte size cap (columns of 206 and 5, minimised from the 413-byte crash). The setup takes no layers as one layer at rate 0 and the cap lifts that rate to 2.4576, as OpenJPEG's does; both encoders write the same lossy stream. The target judged lossless from its own layer count (none asked meant lossless) and not from the rates the setup writes back, so it demanded the image's samples back. It now reads the written-back rate. The encoder was right; `encode_unit` checks the write-back, the oracle's bytes and the oracle's lossy decode of this case. |
