# Direct Windows scatter-only outlining checkpoint

Only scatter_dense inline(always)→inline(never) differs between the two safe crates; all arithmetic, typed MCT and original T1 store are identical. Three processes per case, all samples/status/C++ checks passed.

| Request | New/control: each run | Median new/control | Median new/original |
|---|---:|---:|---:|
| decode / p24_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -1.99% / +0.64% / -0.22% | -0.22% | +2.06% |
| decode / p24_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -1.64% / +2.57% / +1.67% | +1.67% | +5.26% |
| decode / p24_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -2.34% / -4.31% / -0.66% | -2.34% | +3.05% |
| decode / p31_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -2.21% / -2.26% / -1.98% | -2.21% | +1.36% |
| decode / p31_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -3.40% / -2.03% / -1.11% | -2.03% | +2.63% |
| decode / p31_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | +0.77% / -4.18% / -0.41% | -0.41% | +2.64% |
| decode / p31_size_17x37_rgb_97.j2k / r0 / cutfalse / w1 | +1.03% / +1.19% / +1.64% | +1.19% | +1.51% |
| decode / p31_size_17x37_rgb_97.j2k / r1 / cutfalse / w1 | +1.70% / +1.52% / +1.53% | +1.53% | +1.17% |
| decode / p31_size_2x2_rgba_97.j2k / r0 / cutfalse / w1 | -1.13% / -2.23% / -1.53% | -1.53% | -2.49% |
| decode / p31_size_2x2_rgba_97.j2k / r1 / cutfalse / w1 | +1.92% / +1.19% / -0.48% | +1.19% | +1.19% |
| decode / p31_size_512x16_rgb_97_r5.j2k / r0 / cutfalse / w1 | -2.50% / -1.72% / -1.09% | -1.72% | +0.15% |
| decode / p31_size_512x16_rgb_97_r5.j2k / r1 / cutfalse / w1 | -0.67% / -1.29% / -1.10% | -1.10% | -0.11% |
| decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -1.50% / -1.25% / -0.79% | -1.25% | +0.66% |
| decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +0.21% / -0.86% / -0.13% | -0.13% | +2.08% |
| decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | -1.17% / -0.13% / -0.13% | -0.13% | +4.72% |
| packed / sl_128_rgba_97.j2k / r0 / channels3 | -1.50% / -2.16% / -1.79% | -1.79% | +0.11% |
| packed / sl_128_rgba_97.j2k / r0 / channels4 | +0.59% / -0.70% / -1.50% | -0.70% | +1.23% |
| encode / 32 / reversibletrue | +0.33% / +0.02% / +0.36% | +0.33% | +0.65% |
| encode / 32 / reversiblefalse | +0.92% / +0.60% / +1.00% | +0.92% | +0.01% |
