# Direct Windows stride-gated T1 comparison

Both safe versions use the SAME clean per-pixel MCT. Only src/simd/t1.rs differs (manifest.json records the source check). The inherited raw diagnostic header incorrectly says old-safe f64; it is a stale label, not the actual control implementation. Raw logs are preserved unedited. Percent columns are new/old-safe, new/original, old-safe/original. Three processes per case; complete sample/status/C++ checks passed.

| Request | New versus old safe: each run | Median new/old | Median new/original |
|---|---:|---:|---:|
| decode / p24_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -0.08% / -0.77% / -1.28% | -0.77% | +1.65% |
| decode / p24_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +0.01% / +3.63% / +3.63% | +3.63% | +7.69% |
| decode / p24_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | +1.34% / +1.23% / +1.06% | +1.23% | +2.90% |
| decode / p31_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -0.08% / -0.08% / +0.20% | -0.08% | +2.02% |
| decode / p31_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +0.48% / +0.57% / +0.47% | +0.48% | +5.05% |
| decode / p31_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | +0.63% / +0.68% / +0.99% | +0.68% | +2.80% |
| decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +0.62% / +1.04% / +0.48% | +0.62% | +1.54% |
| decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +0.85% / +0.95% / +0.65% | +0.85% | +2.63% |
| decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | +0.34% / +0.91% / +0.61% | +0.61% | +5.48% |
| packed / sl_128_rgba_97.j2k / r0 / channels3 | -1.39% / -0.94% / -0.46% | -0.94% | +2.35% |
| packed / sl_128_rgba_97.j2k / r0 / channels4 | +0.69% / +1.60% / +1.39% | +1.39% | +3.87% |
| encode / 32 / reversibletrue | +1.68% / +4.17% / +1.65% | +1.68% | +1.67% |
| encode / 32 / reversiblefalse | +1.52% / +1.16% / +0.47% | +1.16% | +1.01% |
