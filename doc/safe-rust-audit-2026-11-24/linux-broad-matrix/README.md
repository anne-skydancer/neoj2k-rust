# Linux broad matrix checkpoint

This candidate is safe and passes the checks below, but strict performance preservation remains unresolved. Raw filenames retain the planned `final` prefix; that is a filename, not an acceptance claim.

Three processes per baseline/v3 and full/precision/tails mode:264 row checks. All compare original/candidate/unchanged-control outputs and successful Rust status;72 precision/tail checks additionally compare C++ samples/status. Normal full rows compare original/control outputs; native108 tests separately include C++ differential checks. Full uses defaultfeatures, no stage instrumentation. Source/executable hashes and runner are preserved.

Large p31 v3 remains +2.09% against original while unchanged control is +0.04%; large p24 +1.08% versus control +0.19%. These quiet-control residuals cannot be called preserved performance. Gray controls move materially, so gray timing is inconclusive. Three repeats do not establish platform-wide absence of regression.

## baseline

Percent changes versus original d13082f. Medians are across three process-level paired-median changes. Ranges are the three process results.

| Row | Candidate median | Candidate range | Unchanged control median | Control range |
|---|---:|---:|---:|---:|
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -21.48% | -21.72% .. -21.01% | -0.23% | -0.39% .. +0.31% |
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w2 | -14.10% | -16.75% .. -12.39% | +0.06% | -0.06% .. +0.41% |
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w4 | -10.87% | -13.21% .. -9.79% | -1.62% | -2.73% .. +0.94% |
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w8 | -6.57% | -6.91% .. -5.89% | -0.50% | -0.72% .. +0.20% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w1 | -20.81% | -20.90% .. -20.58% | +4.22% | +2.06% .. +4.55% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w2 | -16.33% | -16.66% .. -15.27% | +5.18% | +4.40% .. +5.40% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w4 | -12.37% | -12.66% .. -12.11% | +3.03% | +2.62% .. +5.44% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w8 | -8.59% | -10.89% .. -6.43% | +3.71% | -1.68% .. +4.85% |
| decode / sl_32_rgba_53.j2k / r0 / cutfalse / w1 | +2.09% | -0.82% .. +2.13% | -2.99% | -5.49% .. -1.99% |
| decode / sl_32_rgba_53.j2k / r2 / cutfalse / w1 | +0.33% | -0.62% .. +0.93% | -0.30% | -0.59% .. +0.21% |
| decode / sl_32_rgba_53.j2k / r0 / cuttrue / w1 | +1.01% | +0.21% .. +1.17% | -3.44% | -4.44% .. -1.77% |
| decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -7.86% | -8.95% .. -7.50% | -0.88% | -2.79% .. -0.18% |
| decode / sl_64_rgb_97.j2k / r2 / cutfalse / w1 | -0.01% | -3.05% .. +1.04% | +1.11% | -1.00% .. +1.59% |
| decode / sl_64_rgb_97.j2k / r0 / cuttrue / w1 | -8.98% | -9.04% .. -8.38% | -0.58% | -1.01% .. +0.25% |
| decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -16.04% | -17.04% .. -15.54% | -1.74% | -2.72% .. -0.13% |
| decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | -2.40% | -2.75% .. -1.54% | +1.03% | +0.24% .. +1.68% |
| decode / sl_128_rgba_97.j2k / r0 / cuttrue / w1 | -16.35% | -17.35% .. -16.33% | -2.54% | -3.11% .. +0.07% |
| encode / 32 / reversibletrue | +1.10% | +0.17% .. +1.23% | -1.98% | -2.99% .. -1.02% |
| encode / 32 / reversiblefalse | +0.16% | -1.66% .. +0.74% | +0.48% | -2.26% .. +0.70% |
| encode / 64 / reversibletrue | +0.23% | -0.26% .. +1.44% | -0.54% | -0.65% .. +0.63% |
| encode / 64 / reversiblefalse | -0.09% | -0.12% .. -0.02% | -0.31% | -0.65% .. +0.00% |
| encode / 128 / reversibletrue | +0.12% | -0.01% .. +1.31% | -0.20% | -0.38% .. -0.15% |
| encode / 128 / reversiblefalse | -0.50% | -0.65% .. -0.08% | -0.26% | -0.32% .. -0.26% |
| packed / sl_128_rgba_97.j2k / r0 / channels1 | -9.03% | -12.31% .. -8.69% | -0.00% | -3.18% .. +0.42% |
| packed / sl_128_rgba_97.j2k / r0 / channels2 | -13.08% | -13.23% .. -11.86% | -1.71% | -2.29% .. +0.58% |
| packed / sl_128_rgba_97.j2k / r0 / channels3 | -12.48% | -12.62% .. -12.38% | -1.87% | -2.12% .. -0.64% |
| packed / sl_128_rgba_97.j2k / r0 / channels4 | -14.41% | -14.44% .. -14.18% | -1.39% | -1.92% .. -0.84% |
| packed / sl_128_rgba_97.j2k / r2 / channels1 | -1.54% | -2.31% .. -0.40% | +0.20% | -0.82% .. +1.15% |
| packed / sl_128_rgba_97.j2k / r2 / channels2 | -2.41% | -3.21% .. -0.80% | -1.21% | -2.70% .. -0.35% |
| packed / sl_128_rgba_97.j2k / r2 / channels3 | +1.17% | -1.54% .. +2.85% | +0.57% | -3.36% .. +2.98% |
| packed / sl_128_rgba_97.j2k / r2 / channels4 | -0.02% | -3.00% .. +0.98% | +4.76% | +2.65% .. +6.35% |
| packed / large_1024_rgb_97.j2k / r0 / channels3 | -20.00% | -20.54% .. -19.62% | +0.09% | -0.53% .. +0.22% |
| decode / p24_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -5.90% | -8.58% .. -5.56% | -2.16% | -2.90% .. -1.71% |
| decode / p24_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -12.18% | -12.85% .. -10.47% | -0.90% | -0.97% .. -0.56% |
| decode / p24_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -15.30% | -15.60% .. -15.24% | +0.13% | +0.03% .. +0.34% |
| decode / p31_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -5.82% | -7.87% .. -5.49% | -1.16% | -3.06% .. -0.00% |
| decode / p31_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -11.84% | -12.09% .. -11.56% | +0.44% | -0.66% .. +0.59% |
| decode / p31_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -14.75% | -14.88% .. -13.80% | +0.23% | -0.18% .. +0.45% |
| decode / p31_size_17x37_rgb_97.j2k / r0 / cutfalse / w1 | -3.29% | -4.61% .. -1.85% | -0.66% | -2.06% .. -0.65% |
| decode / p31_size_17x37_rgb_97.j2k / r1 / cutfalse / w1 | -1.95% | -4.09% .. -1.38% | -0.29% | -2.09% .. +2.96% |
| decode / p31_size_2x2_rgba_97.j2k / r0 / cutfalse / w1 | +2.55% | +1.41% .. +3.28% | -0.10% | -0.69% .. +0.01% |
| decode / p31_size_2x2_rgba_97.j2k / r1 / cutfalse / w1 | +1.75% | +1.45% .. +2.58% | -1.50% | -1.67% .. -1.27% |
| decode / p31_size_512x16_rgb_97_r5.j2k / r0 / cutfalse / w1 | -5.20% | -5.21% .. -4.88% | -0.47% | -0.94% .. +0.66% |
| decode / p31_size_512x16_rgb_97_r5.j2k / r1 / cutfalse / w1 | -2.97% | -3.50% .. -1.78% | +0.82% | +0.53% .. +1.47% |

## v3

Percent changes versus original d13082f. Medians are across three process-level paired-median changes. Ranges are the three process results.

| Row | Candidate median | Candidate range | Unchanged control median | Control range |
|---|---:|---:|---:|---:|
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -2.44% | -3.10% .. -1.83% | +0.03% | -0.26% .. +0.76% |
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w2 | -0.86% | -1.62% .. +0.13% | -1.66% | -1.93% .. -0.58% |
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w4 | +0.29% | -1.05% .. +1.05% | +1.13% | +1.06% .. +1.14% |
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w8 | -2.02% | -2.13% .. -1.68% | -0.28% | -0.50% .. -0.04% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w1 | +13.79% | +12.50% .. +14.65% | +17.05% | +15.53% .. +19.18% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w2 | +14.40% | +13.53% .. +15.61% | +20.19% | +13.00% .. +30.98% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w4 | +8.69% | +7.61% .. +9.32% | +6.09% | +5.74% .. +6.95% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w8 | +9.80% | +6.66% .. +14.24% | +7.92% | +0.06% .. +10.27% |
| decode / sl_32_rgba_53.j2k / r0 / cutfalse / w1 | -0.66% | -3.23% .. +0.97% | -1.60% | -2.27% .. -0.75% |
| decode / sl_32_rgba_53.j2k / r2 / cutfalse / w1 | +0.49% | +0.39% .. +1.72% | +1.11% | +0.90% .. +1.69% |
| decode / sl_32_rgba_53.j2k / r0 / cuttrue / w1 | -1.57% | -1.88% .. -0.97% | -1.51% | -1.61% .. -1.24% |
| decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -0.48% | -0.78% .. +0.58% | +0.04% | -1.82% .. +0.08% |
| decode / sl_64_rgb_97.j2k / r2 / cutfalse / w1 | +1.66% | -0.03% .. +1.72% | +1.07% | +0.74% .. +2.03% |
| decode / sl_64_rgb_97.j2k / r0 / cuttrue / w1 | +0.43% | -2.60% .. +1.39% | -0.39% | -1.85% .. +1.97% |
| decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -0.70% | -0.93% .. -0.39% | +0.71% | +0.09% .. +2.50% |
| decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | +0.56% | +0.51% .. +1.69% | +0.42% | -0.71% .. +0.85% |
| decode / sl_128_rgba_97.j2k / r0 / cuttrue / w1 | -0.34% | -1.19% .. -0.10% | +0.59% | -1.28% .. +0.98% |
| encode / 32 / reversibletrue | -0.58% | -1.51% .. +0.19% | -1.65% | -1.88% .. +0.24% |
| encode / 32 / reversiblefalse | +1.71% | +0.26% .. +4.08% | +1.14% | +0.94% .. +1.89% |
| encode / 64 / reversibletrue | +0.14% | +0.12% .. +0.63% | +0.52% | +0.33% .. +0.63% |
| encode / 64 / reversiblefalse | +0.60% | +0.36% .. +0.91% | +1.06% | +0.65% .. +1.74% |
| encode / 128 / reversibletrue | +0.99% | +0.57% .. +1.35% | +1.38% | +0.88% .. +1.89% |
| encode / 128 / reversiblefalse | +0.83% | +0.57% .. +1.14% | +1.09% | +0.70% .. +1.73% |
| packed / sl_128_rgba_97.j2k / r0 / channels1 | +0.43% | -0.26% .. +3.59% | +2.04% | +1.96% .. +3.68% |
| packed / sl_128_rgba_97.j2k / r0 / channels2 | +0.88% | -0.18% .. +1.04% | +3.41% | +2.29% .. +4.03% |
| packed / sl_128_rgba_97.j2k / r0 / channels3 | -1.73% | -2.17% .. +1.20% | +4.04% | +0.13% .. +4.21% |
| packed / sl_128_rgba_97.j2k / r0 / channels4 | -1.35% | -1.80% .. +0.35% | +0.51% | -2.31% .. +4.80% |
| packed / sl_128_rgba_97.j2k / r2 / channels1 | +1.96% | -0.79% .. +2.80% | -0.45% | -1.42% .. +1.75% |
| packed / sl_128_rgba_97.j2k / r2 / channels2 | +1.58% | +1.56% .. +3.99% | +1.54% | -0.32% .. +2.47% |
| packed / sl_128_rgba_97.j2k / r2 / channels3 | +2.71% | +1.64% .. +3.75% | +4.05% | +3.69% .. +4.83% |
| packed / sl_128_rgba_97.j2k / r2 / channels4 | +3.85% | +3.54% .. +6.16% | +6.89% | +5.22% .. +12.49% |
| packed / large_1024_rgb_97.j2k / r0 / channels3 | -3.38% | -3.45% .. -3.30% | -0.46% | -1.41% .. -0.38% |
| decode / p24_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +2.34% | +0.99% .. +3.36% | +0.52% | -1.37% .. +0.63% |
| decode / p24_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +1.00% | -0.77% .. +1.19% | +1.99% | +1.31% .. +2.35% |
| decode / p24_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | +1.08% | +0.29% .. +1.20% | +0.19% | +0.06% .. +0.35% |
| decode / p31_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +0.87% | +0.66% .. +2.92% | +0.77% | -1.37% .. +1.98% |
| decode / p31_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +2.27% | +0.56% .. +2.35% | -0.32% | -1.08% .. +1.94% |
| decode / p31_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | +2.09% | +1.58% .. +2.35% | +0.04% | -0.08% .. +0.55% |
| decode / p31_size_17x37_rgb_97.j2k / r0 / cutfalse / w1 | +1.84% | -0.03% .. +3.05% | +0.99% | -0.48% .. +3.37% |
| decode / p31_size_17x37_rgb_97.j2k / r1 / cutfalse / w1 | +3.57% | +2.17% .. +4.70% | +0.17% | -0.09% .. +1.29% |
| decode / p31_size_2x2_rgba_97.j2k / r0 / cutfalse / w1 | +6.25% | +6.03% .. +6.37% | +4.79% | +4.72% .. +5.16% |
| decode / p31_size_2x2_rgba_97.j2k / r1 / cutfalse / w1 | +5.69% | +4.39% .. +7.13% | +7.88% | +7.34% .. +8.90% |
| decode / p31_size_512x16_rgb_97_r5.j2k / r0 / cutfalse / w1 | -0.27% | -0.38% .. +0.27% | +1.01% | -0.48% .. +1.35% |
| decode / p31_size_512x16_rgb_97_r5.j2k / r1 / cutfalse / w1 | -0.01% | -0.19% .. +0.05% | +0.05% | -1.05% .. +0.87% |
