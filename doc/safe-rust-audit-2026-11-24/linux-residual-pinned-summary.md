# baseline

| Request | Candidate vs base | Repetition range | Control vs base | Control range |
|---|---:|---:|---:|---:|
| decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -7.347% | -7.398% to -5.018% | +2.645% | +1.501% to +2.847% |
| decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -15.865% | -17.465% to -14.319% | -1.924% | -3.180% to +2.340% |
| decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | +0.999% | -1.182% to +1.275% | +0.878% | -1.550% to +1.525% |
| packed / sl_128_rgba_97.j2k / r0 / channels3 | -11.942% | -12.521% to -10.202% | +0.743% | -1.317% to +0.813% |
| packed / sl_128_rgba_97.j2k / r0 / channels4 | -12.645% | -16.394% to -12.109% | -0.688% | -2.209% to -0.231% |
| encode / 32 / reversibletrue | -0.632% | -0.864% to +4.139% | -1.542% | -2.256% to -0.979% |
| encode / 32 / reversiblefalse | +3.261% | +1.898% to +4.136% | -2.405% | -2.437% to +0.656% |

# v3

| Request | Candidate vs base | Repetition range | Control vs base | Control range |
|---|---:|---:|---:|---:|
| decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +7.613% | +6.211% to +8.438% | -0.394% | -1.065% to +1.754% |
| decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +3.861% | +2.870% to +5.952% | +0.600% | -0.903% to +1.922% |
| decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | +2.535% | +0.480% to +3.200% | +1.595% | +0.904% to +3.226% |
| packed / sl_128_rgba_97.j2k / r0 / channels3 | +4.436% | -0.502% to +11.290% | -0.871% | -5.069% to +1.136% |
| packed / sl_128_rgba_97.j2k / r0 / channels4 | +5.669% | +4.608% to +6.331% | +0.881% | -1.867% to +2.299% |
| encode / 32 / reversibletrue | -1.293% | -3.391% to -0.951% | +0.982% | -3.783% to +2.717% |
| encode / 32 / reversiblefalse | +0.863% | -1.700% to +3.024% | -1.369% | -1.527% to -1.155% |
