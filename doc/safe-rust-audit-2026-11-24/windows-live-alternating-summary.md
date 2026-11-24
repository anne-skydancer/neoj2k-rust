# Windows v3 alternating precision checkpoint

Three alternating executable pairs; each executable internally compares candidate/original/unchanged control. This is not a direct same-process old-safe/new-safe comparison. Values below are medians of each process's paired ratio. All six requests in all processes passed complete sample/status/C++ checks.

| Case | f64 vs original | Clean vs original | f64 control | Clean control |
|---|---:|---:|---:|---:|
| p24_sl_64_rgb_97.j2k | +4.04% | +3.43% | +0.15% | +1.36% |
| p24_sl_128_rgba_97.j2k | +8.28% | +7.08% | +0.96% | +0.04% |
| p24_large_1024_rgb_97.j2k | +4.03% | +2.02% | -0.30% | -0.51% |
| p31_sl_64_rgb_97.j2k | +3.37% | +3.48% | +1.15% | +1.61% |
| p31_sl_128_rgba_97.j2k | +5.69% | +5.21% | +2.96% | +4.45% |
| p31_large_1024_rgb_97.j2k | +4.14% | +1.90% | -0.43% | -0.26% |
