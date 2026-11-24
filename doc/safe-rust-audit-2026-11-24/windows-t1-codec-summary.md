# Windows T1 hybrid codec checkpoint

Alternating executables, each internally comparing candidate/original/control. Three runs per version. This is diagnostic, not a direct same-process pre/post-safe comparison. All sample/status checks passed.

| Case | Pre-T1 vs original | Hybrid vs original | Pre control | Hybrid control |
|---|---:|---:|---:|---:|
| decode / p24_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +1.47% | +1.06% | -0.15% | +0.29% |
| decode / p24_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +7.51% | +6.81% | -0.18% | -0.25% |
| decode / p24_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | +1.91% | +3.07% | -0.17% | +0.10% |
| decode / p31_sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +3.77% | +3.28% | +1.88% | +0.85% |
| decode / p31_sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +4.84% | +5.29% | +0.55% | +0.56% |
| decode / p31_large_1024_rgb_97.j2k / r0 / cutfalse / w1 | +1.78% | +3.00% | -0.38% | +0.20% |
| decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -0.08% | -0.28% | -0.75% | -1.30% |
| decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +3.80% | +0.86% | +4.28% | +1.36% |
| decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | +4.64% | +4.11% | +5.93% | +4.46% |
| packed / sl_128_rgba_97.j2k / r0 / channels3 | +4.02% | +2.69% | +5.06% | +6.37% |
| packed / sl_128_rgba_97.j2k / r0 / channels4 | +3.00% | +0.95% | +6.07% | +4.29% |
| encode / 32 / reversibletrue | +0.35% | +0.43% | -0.89% | +0.30% |
| encode / 32 / reversiblefalse | -1.01% | -0.24% | -0.47% | +0.05% |
