# baseline

| Request | Candidate vs base | Repetition range | Control vs base | Control range |
|---|---:|---:|---:|---:|
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -21.527% | -21.688% to -21.134% | -0.336% | -0.849% to +0.640% |
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w2 | -15.826% | -16.533% to -14.346% | -1.920% | -2.070% to -1.151% |
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w4 | -11.258% | -12.684% to -9.973% | -1.532% | -1.936% to -0.021% |
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w8 | -7.389% | -7.579% to -7.176% | -1.352% | -2.132% to -0.483% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w1 | -21.902% | -22.104% to -19.774% | +1.550% | +0.620% to +2.320% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w2 | -17.194% | -18.233% to -16.958% | +3.463% | +2.379% to +4.165% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w4 | -13.871% | -15.119% to -13.615% | +2.783% | +0.832% to +4.447% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w8 | -12.575% | -14.310% to -10.567% | -0.881% | -1.022% to +1.488% |
| decode / sl_32_rgba_53.j2k / r0 / cutfalse / w1 | +3.485% | +1.072% to +6.127% | +1.647% | +0.884% to +1.955% |
| decode / sl_32_rgba_53.j2k / r2 / cutfalse / w1 | +0.485% | -0.637% to +0.900% | -0.779% | -0.979% to -0.775% |
| decode / sl_32_rgba_53.j2k / r0 / cuttrue / w1 | +3.426% | +2.893% to +3.742% | -1.102% | -2.077% to +1.840% |
| decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | -6.958% | -8.220% to -6.177% | +1.958% | +0.508% to +3.956% |
| decode / sl_64_rgb_97.j2k / r2 / cutfalse / w1 | +0.476% | -2.081% to +0.888% | +3.218% | +2.002% to +3.247% |
| decode / sl_64_rgb_97.j2k / r0 / cuttrue / w1 | -5.182% | -9.443% to -4.689% | +1.262% | +0.720% to +2.655% |
| decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | -15.441% | -18.799% to -15.217% | -1.415% | -2.812% to -0.580% |
| decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | -0.718% | -2.975% to +0.742% | +2.345% | +2.340% to +3.829% |
| decode / sl_128_rgba_97.j2k / r0 / cuttrue / w1 | -15.456% | -15.672% to -14.330% | -0.086% | -0.211% to +3.065% |
| encode / 32 / reversibletrue | +0.350% | +0.324% to +1.599% | -0.612% | -3.494% to +0.036% |
| encode / 32 / reversiblefalse | +4.377% | +3.710% to +5.364% | +0.088% | -0.821% to +2.535% |
| encode / 64 / reversibletrue | -0.479% | -0.649% to -0.439% | -0.786% | -1.140% to -0.562% |
| encode / 64 / reversiblefalse | +1.207% | +0.748% to +1.790% | -0.856% | -1.155% to +1.229% |
| encode / 128 / reversibletrue | -0.345% | -1.740% to -0.282% | +0.136% | -0.465% to +1.801% |
| encode / 128 / reversiblefalse | +1.148% | -0.143% to +1.388% | -0.757% | -1.004% to +0.059% |
| packed / sl_128_rgba_97.j2k / r0 / channels1 | -8.164% | -11.251% to -7.542% | +3.148% | +0.528% to +6.654% |
| packed / sl_128_rgba_97.j2k / r0 / channels2 | -10.592% | -11.558% to -10.236% | -1.251% | -2.124% to +0.404% |
| packed / sl_128_rgba_97.j2k / r0 / channels3 | -11.361% | -12.452% to -10.415% | -0.137% | -1.648% to +2.961% |
| packed / sl_128_rgba_97.j2k / r0 / channels4 | -12.843% | -12.935% to -12.809% | -0.116% | -1.524% to +1.204% |
| packed / sl_128_rgba_97.j2k / r2 / channels1 | -1.156% | -3.228% to -0.793% | -0.429% | -0.458% to +2.291% |
| packed / sl_128_rgba_97.j2k / r2 / channels2 | +0.185% | -2.365% to +1.286% | +1.842% | -0.505% to +3.427% |
| packed / sl_128_rgba_97.j2k / r2 / channels3 | +0.201% | -0.535% to +0.819% | +1.802% | +1.118% to +6.875% |
| packed / sl_128_rgba_97.j2k / r2 / channels4 | +1.963% | -0.038% to +4.997% | +5.317% | -6.285% to +7.564% |
| packed / large_1024_rgb_97.j2k / r0 / channels3 | -21.555% | -22.054% to -20.688% | -1.080% | -1.187% to -0.810% |

# v3

| Request | Candidate vs base | Repetition range | Control vs base | Control range |
|---|---:|---:|---:|---:|
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w1 | -2.452% | -2.605% to -1.285% | +0.045% | +0.032% to +0.062% |
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w2 | +0.079% | +0.036% to +3.396% | -0.528% | -0.992% to +0.151% |
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w4 | +2.019% | +0.667% to +2.073% | +2.341% | +1.588% to +2.796% |
| decode / large_1024_rgb_97.j2k / r0 / cutfalse / w8 | -0.259% | -1.114% to +0.431% | +0.912% | -0.177% to +1.166% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w1 | +12.577% | +12.409% to +13.534% | +12.472% | +11.824% to +13.211% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w2 | +11.834% | +8.084% to +13.847% | +12.913% | +9.975% to +13.348% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w4 | +6.893% | +4.027% to +8.992% | +8.391% | +7.142% to +10.442% |
| decode / large_4096_g_97.j2k / r0 / cutfalse / w8 | +4.506% | +0.717% to +5.919% | +2.024% | -2.182% to +2.314% |
| decode / sl_32_rgba_53.j2k / r0 / cutfalse / w1 | -1.622% | -2.285% to +0.439% | -1.366% | -2.143% to +1.284% |
| decode / sl_32_rgba_53.j2k / r2 / cutfalse / w1 | +0.319% | +0.000% to +4.280% | -0.285% | -0.490% to +2.533% |
| decode / sl_32_rgba_53.j2k / r0 / cuttrue / w1 | -1.150% | -1.455% to -0.428% | -0.343% | -10.738% to -0.170% |
| decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1 | +7.275% | +4.363% to +7.821% | -2.902% | -6.652% to +1.409% |
| decode / sl_64_rgb_97.j2k / r2 / cutfalse / w1 | +1.543% | -1.110% to +1.899% | -0.638% | -1.069% to +2.825% |
| decode / sl_64_rgb_97.j2k / r0 / cuttrue / w1 | +6.983% | +5.890% to +9.186% | -2.037% | -2.255% to -1.305% |
| decode / sl_128_rgba_97.j2k / r0 / cutfalse / w1 | +2.893% | +1.790% to +4.290% | +0.274% | -0.887% to +4.809% |
| decode / sl_128_rgba_97.j2k / r2 / cutfalse / w1 | +2.184% | +0.215% to +2.359% | -0.004% | -1.345% to +2.695% |
| decode / sl_128_rgba_97.j2k / r0 / cuttrue / w1 | +4.454% | -0.060% to +6.395% | -0.560% | -1.369% to +1.583% |
| encode / 32 / reversibletrue | -0.238% | -1.053% to +0.984% | -2.221% | -2.534% to -0.858% |
| encode / 32 / reversiblefalse | +0.297% | -0.260% to +0.836% | -0.988% | -1.424% to +2.471% |
| encode / 64 / reversibletrue | -0.141% | -1.612% to +0.232% | -0.236% | -0.859% to +1.183% |
| encode / 64 / reversiblefalse | +1.793% | +1.205% to +2.716% | -0.605% | -0.694% to -0.531% |
| encode / 128 / reversibletrue | -0.121% | -1.174% to +0.043% | -0.562% | -1.064% to +0.361% |
| encode / 128 / reversiblefalse | +0.615% | -0.796% to +1.452% | -1.081% | -2.226% to -0.917% |
| packed / sl_128_rgba_97.j2k / r0 / channels1 | +2.964% | +2.631% to +5.798% | +0.022% | -1.145% to +3.635% |
| packed / sl_128_rgba_97.j2k / r0 / channels2 | +4.042% | +3.394% to +6.837% | +2.706% | -1.492% to +4.208% |
| packed / sl_128_rgba_97.j2k / r0 / channels3 | +5.346% | +3.636% to +6.017% | +1.924% | -5.632% to +2.377% |
| packed / sl_128_rgba_97.j2k / r0 / channels4 | +1.395% | +1.164% to +5.133% | -1.514% | -2.487% to +0.554% |
| packed / sl_128_rgba_97.j2k / r2 / channels1 | +0.773% | -1.212% to +3.702% | -0.976% | -1.496% to +1.129% |
| packed / sl_128_rgba_97.j2k / r2 / channels2 | +2.013% | +1.290% to +2.201% | -0.435% | -4.210% to -0.347% |
| packed / sl_128_rgba_97.j2k / r2 / channels3 | +3.879% | +2.917% to +6.202% | +6.670% | +3.458% to +7.362% |
| packed / sl_128_rgba_97.j2k / r2 / channels4 | +4.031% | +2.672% to +8.773% | +1.239% | +1.220% to +6.497% |
| packed / large_1024_rgb_97.j2k / r0 / channels3 | -0.852% | -1.177% to -0.514% | +1.692% | +1.479% to +1.737% |
