# Direct Windows typed colour and unary finishing checkpoint

Only simd/mct.rs differs between the safe versions. Control uses untyped three-op colour and f64 unary finishing. Original coefficient store and DWT are unchanged on both sides. Three processes per request; all complete samples/status/C++ checks passed.

| Request | New/control: each run | Median new/control | Median new/original |
|---|---:|---:|---:|
| decode / p24_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -1.98% / +0.05% / -0.20% | -0.20% | +1.59% |
| decode / p24_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -0.14% / +3.21% / +1.90% | +1.90% | +4.92% |
| decode / p24_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -1.02% / -1.21% / -1.20% | -1.20% | -0.06% |
| decode / p31_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -0.76% / -1.89% / +0.49% | -0.76% | +1.13% |
| decode / p31_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -2.70% / -3.62% / -3.87% | -3.62% | +1.89% |
| decode / p31_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -0.91% / -0.97% / -1.25% | -0.97% | +0.07% |
| decode / p31_size_17x37_rgb_97.j2k / r0 / cutfalse / w1 | -0.77% / -0.98% / -1.14% | -0.98% | -1.47% |
| decode / p31_size_17x37_rgb_97.j2k / r1 / cutfalse / w1 | -2.48% / -0.79% / -1.38% | -1.38% | -2.69% |
| decode / p31_size_2x2_rgba_97.j2k / r0 / cutfalse / w1 | -1.13% / -0.85% / -1.41% | -1.13% | -0.71% |
| decode / p31_size_2x2_rgba_97.j2k / r1 / cutfalse / w1 | -2.34% / -1.19% / -1.42% | -1.42% | +0.24% |
| decode / p31_size_512x16_rgb_97_r5.j2k / r0 / cutfalse / w1 | +1.22% / +0.24% / -0.01% | +0.24% | +1.35% |
| decode / p31_size_512x16_rgb_97_r5.j2k / r1 / cutfalse / w1 | -0.72% / -0.60% / -0.53% | -0.60% | +0.65% |
| decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -0.42% / -1.10% / -0.35% | -0.42% | +1.70% |
| decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +0.84% / +4.49% / -0.51% | +0.84% | -0.31% |
| decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | +0.35% / +0.25% / +0.22% | +0.25% | +0.92% |
| packed / sl_128_rgba_97.j2k / r0 / channels3 | +0.02% / +0.31% / +0.74% | +0.31% | +4.31% |
| packed / sl_128_rgba_97.j2k / r0 / channels4 | +0.19% / -0.42% / +0.82% | +0.19% | +0.48% |
| encode / 32 / reversibletrue | -0.17% / -0.83% / -0.52% | -0.52% | -0.10% |
| encode / 32 / reversiblefalse | +1.78% / +1.12% / +0.65% | +1.12% | -0.34% |
