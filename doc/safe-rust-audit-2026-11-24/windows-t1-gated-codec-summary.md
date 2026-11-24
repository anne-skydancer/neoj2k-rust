# Windows stride-gated T1 checkpoint

Three alternating executable pairs; each internally compares candidate/original/unchanged control. These are not direct same-process pre/post-safe comparisons. All samples/status checks passed.

| Case | Pre-T1 vs original | Gated vs original | Pre control | Gated control |
|---|---:|---:|---:|---:|
| decode / p24_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +3.18% | +2.00% | +2.08% | +2.17% |
| decode / p24_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +7.16% | +3.96% | +0.41% | -0.40% |
| decode / p24_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | +0.46% | +0.17% | -2.22% | -2.36% |
| decode / p31_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +2.41% | +4.02% | +2.98% | +1.93% |
| decode / p31_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +3.61% | +3.23% | +0.67% | +0.57% |
| decode / p31_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -0.29% | +1.31% | -2.89% | -2.21% |
| decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +0.54% | +0.89% | -0.45% | -0.80% |
| decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +2.10% | +3.88% | +2.69% | +4.64% |
| decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | +3.68% | +1.36% | +5.51% | +1.46% |
| packed / sl_128_rgba_97.j2k / r0 / channels3 | +3.70% | +1.94% | +5.53% | +2.40% |
| packed / sl_128_rgba_97.j2k / r0 / channels4 | +1.13% | +2.91% | +1.51% | +2.72% |
| encode / 32 / reversibletrue | +0.38% | +0.81% | -1.17% | -2.25% |
| encode / 32 / reversiblefalse | -0.28% | -0.03% | -1.46% | -0.49% |
