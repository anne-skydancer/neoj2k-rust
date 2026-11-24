# Frozen candidate Windows matrix

Three processes per baseline/v3 cell and mode. Values are medians of the processes' paired percentage changes, not ratios of unrelated absolute medians. All264 rows compare Rust outputs/status;72 precision/tail rows also check C++ samples/status. Source/executable hashes are in ../final-candidate-checkpoint/windows-manifest.json.

| Cell | Request | Candidate/original | Unchanged control/original |
|---|---|---:|---:|
| baseline | decode / large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -21.36% | -1.46% |
| baseline | decode / large_1024_rgb_97.j2k / r0 / cutfalse / w2 | -18.16% | -0.07% |
| baseline | decode / large_1024_rgb_97.j2k / r0 / cutfalse / w4 | -17.70% | -0.57% |
| baseline | decode / large_1024_rgb_97.j2k / r0 / cutfalse / w8 | -12.14% | -0.78% |
| baseline | decode / large_4096_g_97.j2k / r0 / cutfalse / w1 | -23.49% | -0.06% |
| baseline | decode / large_4096_g_97.j2k / r0 / cutfalse / w2 | -21.91% | -0.76% |
| baseline | decode / large_4096_g_97.j2k / r0 / cutfalse / w4 | -18.29% | -0.15% |
| baseline | decode / large_4096_g_97.j2k / r0 / cutfalse / w8 | -13.04% | +2.59% |
| baseline | decode / sl_32_rgba_53.j2k / r0 / cutfalse / w1 | +1.25% | -0.75% |
| baseline | decode / sl_32_rgba_53.j2k / r2 / cutfalse / w1 | +0.69% | +0.98% |
| baseline | decode / sl_32_rgba_53.j2k / r0 / cuttrue / w1 | +0.39% | +0.20% |
| baseline | decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -6.01% | +3.70% |
| baseline | decode / sl_64_rgb_97.j2k / r2 / cutfalse / w1 | -1.94% | +0.16% |
| baseline | decode / sl_64_rgb_97.j2k / r0 / cuttrue / w1 | -4.75% | -1.15% |
| baseline | decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -14.51% | -0.13% |
| baseline | decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | -0.83% | +4.38% |
| baseline | decode / sl_128_rgba_97.j2k / r0 / cuttrue / w1 | -15.17% | -1.03% |
| baseline | encode / 32 / reversibletrue | +2.29% | -2.44% |
| baseline | encode / 32 / reversiblefalse | +2.10% | +0.20% |
| baseline | encode / 64 / reversibletrue | +0.61% | +0.97% |
| baseline | encode / 64 / reversiblefalse | +0.23% | +1.18% |
| baseline | encode / 128 / reversibletrue | +0.17% | -0.10% |
| baseline | encode / 128 / reversiblefalse | +0.22% | +1.69% |
| baseline | packed / sl_128_rgba_97.j2k / r0 / channels1 | -10.51% | -1.62% |
| baseline | packed / sl_128_rgba_97.j2k / r0 / channels2 | -10.44% | -2.26% |
| baseline | packed / sl_128_rgba_97.j2k / r0 / channels3 | -9.62% | +0.36% |
| baseline | packed / sl_128_rgba_97.j2k / r0 / channels4 | -12.47% | -0.40% |
| baseline | packed / sl_128_rgba_97.j2k / r2 / channels1 | -1.33% | +0.56% |
| baseline | packed / sl_128_rgba_97.j2k / r2 / channels2 | -1.17% | +2.25% |
| baseline | packed / sl_128_rgba_97.j2k / r2 / channels3 | +2.74% | +3.73% |
| baseline | packed / sl_128_rgba_97.j2k / r2 / channels4 | +4.09% | +1.26% |
| baseline | packed / large_1024_rgb_97.j2k / r0 / channels3 | -18.87% | -0.17% |
| baseline | decode / p24_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -4.26% | +1.10% |
| baseline | decode / p24_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -10.87% | +0.34% |
| baseline | decode / p24_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -15.37% | -0.81% |
| baseline | decode / p31_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -4.79% | +1.22% |
| baseline | decode / p31_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -10.28% | +0.63% |
| baseline | decode / p31_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -15.68% | -1.15% |
| baseline | decode / p31_size_17x37_rgb_97.j2k / r0 / cutfalse / w1 | -3.68% | -0.87% |
| baseline | decode / p31_size_17x37_rgb_97.j2k / r1 / cutfalse / w1 | -2.34% | +0.18% |
| baseline | decode / p31_size_2x2_rgba_97.j2k / r0 / cutfalse / w1 | +1.16% | +1.74% |
| baseline | decode / p31_size_2x2_rgba_97.j2k / r1 / cutfalse / w1 | +0.48% | +1.45% |
| baseline | decode / p31_size_512x16_rgb_97_r5.j2k / r0 / cutfalse / w1 | -4.73% | +0.97% |
| baseline | decode / p31_size_512x16_rgb_97_r5.j2k / r1 / cutfalse / w1 | -3.13% | +1.36% |
| v3 | decode / large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -1.63% | +0.34% |
| v3 | decode / large_1024_rgb_97.j2k / r0 / cutfalse / w2 | -2.51% | +0.07% |
| v3 | decode / large_1024_rgb_97.j2k / r0 / cutfalse / w4 | -0.87% | -0.24% |
| v3 | decode / large_1024_rgb_97.j2k / r0 / cutfalse / w8 | -0.83% | +0.16% |
| v3 | decode / large_4096_g_97.j2k / r0 / cutfalse / w1 | -2.41% | +2.04% |
| v3 | decode / large_4096_g_97.j2k / r0 / cutfalse / w2 | +0.39% | +0.29% |
| v3 | decode / large_4096_g_97.j2k / r0 / cutfalse / w4 | +1.45% | +3.20% |
| v3 | decode / large_4096_g_97.j2k / r0 / cutfalse / w8 | +3.47% | +2.33% |
| v3 | decode / sl_32_rgba_53.j2k / r0 / cutfalse / w1 | +0.19% | -3.57% |
| v3 | decode / sl_32_rgba_53.j2k / r2 / cutfalse / w1 | +0.02% | +0.08% |
| v3 | decode / sl_32_rgba_53.j2k / r0 / cuttrue / w1 | +1.78% | -1.13% |
| v3 | decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +0.39% | +1.42% |
| v3 | decode / sl_64_rgb_97.j2k / r2 / cutfalse / w1 | -0.12% | +0.76% |
| v3 | decode / sl_64_rgb_97.j2k / r0 / cuttrue / w1 | +2.10% | -0.30% |
| v3 | decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +0.89% | +1.55% |
| v3 | decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | +1.64% | +5.78% |
| v3 | decode / sl_128_rgba_97.j2k / r0 / cuttrue / w1 | +1.38% | +3.00% |
| v3 | encode / 32 / reversibletrue | +0.78% | -0.87% |
| v3 | encode / 32 / reversiblefalse | -1.22% | +0.54% |
| v3 | encode / 64 / reversibletrue | +0.42% | -0.58% |
| v3 | encode / 64 / reversiblefalse | -0.42% | -0.91% |
| v3 | encode / 128 / reversibletrue | -0.06% | +0.21% |
| v3 | encode / 128 / reversiblefalse | -0.41% | -0.57% |
| v3 | packed / sl_128_rgba_97.j2k / r0 / channels1 | +1.89% | +4.17% |
| v3 | packed / sl_128_rgba_97.j2k / r0 / channels2 | +3.36% | +3.61% |
| v3 | packed / sl_128_rgba_97.j2k / r0 / channels3 | +3.56% | +2.68% |
| v3 | packed / sl_128_rgba_97.j2k / r0 / channels4 | +2.63% | +5.01% |
| v3 | packed / sl_128_rgba_97.j2k / r2 / channels1 | +0.63% | -0.13% |
| v3 | packed / sl_128_rgba_97.j2k / r2 / channels2 | +2.93% | +1.16% |
| v3 | packed / sl_128_rgba_97.j2k / r2 / channels3 | +2.96% | +0.66% |
| v3 | packed / sl_128_rgba_97.j2k / r2 / channels4 | +3.86% | -4.24% |
| v3 | packed / large_1024_rgb_97.j2k / r0 / channels3 | -0.63% | +0.23% |
| v3 | decode / p24_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +0.78% | +0.31% |
| v3 | decode / p24_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +5.70% | +2.13% |
| v3 | decode / p24_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | +1.92% | +0.55% |
| v3 | decode / p31_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +1.95% | -1.35% |
| v3 | decode / p31_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +3.19% | +0.48% |
| v3 | decode / p31_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | +1.49% | +0.24% |
| v3 | decode / p31_size_17x37_rgb_97.j2k / r0 / cutfalse / w1 | -0.62% | -0.75% |
| v3 | decode / p31_size_17x37_rgb_97.j2k / r1 / cutfalse / w1 | -1.42% | -1.95% |
| v3 | decode / p31_size_2x2_rgba_97.j2k / r0 / cutfalse / w1 | +0.14% | -1.25% |
| v3 | decode / p31_size_2x2_rgba_97.j2k / r1 / cutfalse / w1 | +0.24% | +0.47% |
| v3 | decode / p31_size_512x16_rgb_97_r5.j2k / r0 / cutfalse / w1 | -0.63% | -0.07% |
| v3 | decode / p31_size_512x16_rgb_97_r5.j2k / r1 / cutfalse / w1 | +0.48% | +0.34% |
