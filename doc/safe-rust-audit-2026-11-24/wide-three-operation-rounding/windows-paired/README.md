# Direct Windows three-operation rounding checkpoint

The safe versions differ only in simd/mct.rs, with the same original coefficient store. Columns compare three-op against four-op, then original d13082f. Three processes; all complete samples/status/C++ checks passed. This checkpoint does not establish platform-wide performance preservation.

| Request | Three-op/four-op: each run | Median three/four | Median three/original |
|---|---:|---:|---:|
| decode / p24_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -0.16% / +0.55% / +0.84% | +0.55% | +1.02% |
| decode / p24_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +0.15% / +2.52% / +1.12% | +1.12% | +5.68% |
| decode / p24_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -0.96% / -0.93% / -0.61% | -0.93% | +1.71% |
| decode / p31_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -0.77% / -0.18% / -0.08% | -0.18% | +0.26% |
| decode / p31_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -2.75% / -1.01% / +0.09% | -1.01% | +3.64% |
| decode / p31_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -0.64% / -0.59% / -0.87% | -0.64% | +2.00% |
| decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -1.69% / -0.92% / +1.14% | -0.92% | -0.07% |
| decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -0.14% / +0.17% / +0.65% | +0.17% | -1.29% |
| decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | -0.60% / +1.05% / +0.48% | +0.48% | -1.28% |
| packed / sl_128_rgba_97.j2k / r0 / channels3 | -0.09% / -0.57% / -1.01% | -0.57% | +1.31% |
| packed / sl_128_rgba_97.j2k / r0 / channels4 | -0.71% / -1.06% / -1.72% | -1.06% | +0.46% |
| encode / 32 / reversibletrue | +0.40% / +0.10% / +1.00% | +0.40% | +0.65% |
| encode / 32 / reversiblefalse | +0.87% / +0.87% / +0.59% | +0.87% | +0.45% |
