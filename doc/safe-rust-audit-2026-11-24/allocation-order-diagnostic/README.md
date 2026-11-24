# Runtime allocation-order diagnostic

One executable, six constructor/warm permutations, two passes in differing order; timed call order remains balanced. Public original/candidate/unchanged-control checks pass for72 rows, including36 precision rows additionally matched to C++. All48 decode preparations logged the expected constructor order. Destination addresses are preserved. No production source changed.

The candidate unsigned colour helper is708 bytes/148 normalized instructions in both broad-matrix and diagnostic executables, with identical instruction sequences after relocation addresses are removed. Its code placement is fixed throughout the runtime-order diagnostic. The different wrapper executable still has different global placement; this experiment does not separate every cache, allocator and placement effect.

Largep31 changes materially with context, while smallp31 stays positive in all six order medians. This does not prove absence of regression or excuse the broad-matrix quiet-control gap. Two repeats/order are diagnostic, not acceptance evidence.

| Order | Construction order |
|---|---|
| 0 | base → candidate → control |
| 1 | control → candidate → base |
| 2 | candidate → base → control |
| 3 | control → base → candidate |
| 4 | base → control → candidate |
| 5 | candidate → control → base |

## decode / p31_large_1024_rgb_97.j2k / r0 / cutfalse / w1

Percent versus original; two process-phase results per order.

| Order | Candidate median | Candidate individual | Unchanged control median | Control individual |
|---|---:|---|---:|---|
| 0 | -0.439% | -0.181%, -0.696% | -0.321% | -0.563%, -0.080% |
| 1 | -0.109% | +0.826%, -1.045% | +0.430% | +0.549%, +0.311% |
| 2 | +0.791% | -0.290%, +1.873% | -0.051% | -0.762%, +0.659% |
| 3 | +0.922% | -0.535%, +2.380% | +0.424% | -0.416%, +1.265% |
| 4 | +0.989% | +0.798%, +1.179% | -1.543% | -1.478%, -1.609% |
| 5 | +0.526% | +0.113%, +0.940% | -1.315% | -1.519%, -1.111% |

## decode / p24_sl_128_rgba_97.j2k / r0 / cutfalse / w1

Percent versus original; two process-phase results per order.

| Order | Candidate median | Candidate individual | Unchanged control median | Control individual |
|---|---:|---|---:|---|
| 0 | +1.064% | +1.422%, +0.706% | +0.619% | +1.795%, -0.557% |
| 1 | +1.855% | +2.181%, +1.529% | -1.307% | -0.911%, -1.703% |
| 2 | +1.918% | +2.482%, +1.354% | -0.364% | -1.852%, +1.125% |
| 3 | +1.185% | +1.207%, +1.163% | +0.274% | -0.063%, +0.612% |
| 4 | +1.426% | +0.530%, +2.322% | -1.722% | -1.813%, -1.631% |
| 5 | +0.418% | +1.506%, -0.669% | -0.817% | +0.152%, -1.785% |

## decode / p31_sl_128_rgba_97.j2k / r0 / cutfalse / w1

Percent versus original; two process-phase results per order.

| Order | Candidate median | Candidate individual | Unchanged control median | Control individual |
|---|---:|---|---:|---|
| 0 | +1.927% | +1.710%, +2.145% | +0.504% | +0.877%, +0.131% |
| 1 | +2.268% | +3.034%, +1.502% | -0.866% | -0.688%, -1.045% |
| 2 | +1.796% | +1.408%, +2.184% | -3.274% | -0.863%, -5.686% |
| 3 | +2.580% | +2.778%, +2.383% | -0.095% | -0.239%, +0.049% |
| 4 | +2.433% | +2.293%, +2.572% | +0.661% | +1.138%, +0.184% |
| 5 | +0.629% | -0.281%, +1.539% | +0.388% | -1.599%, +2.376% |

## decode / sl_64_rgb_97.j2k / r0 / cutfalse / w1

Percent versus original; two process-phase results per order.

| Order | Candidate median | Candidate individual | Unchanged control median | Control individual |
|---|---:|---|---:|---|
| 0 | +2.481% | +2.300%, +2.662% | -0.586% | +0.509%, -1.681% |
| 1 | +1.016% | +0.689%, +1.343% | +0.227% | +0.655%, -0.201% |
| 2 | +0.714% | +0.484%, +0.945% | -2.415% | -2.550%, -2.280% |
| 3 | +0.246% | +0.346%, +0.146% | -1.962% | -0.346%, -3.578% |
| 4 | +0.475% | +2.733%, -1.784% | +1.297% | +3.399%, -0.805% |
| 5 | +3.465% | +2.862%, +4.067% | +2.198% | +1.608%, +2.788% |

## encode / 32 / reversibletrue

Percent versus original; two process-phase results per order.

| Order | Candidate median | Candidate individual | Unchanged control median | Control individual |
|---|---:|---|---:|---|
| 0 | +1.450% | +1.750%, +1.150% | -0.351% | +0.484%, -1.186% |
| 1 | +0.242% | +0.254%, +0.231% | -1.659% | -0.196%, -3.121% |
| 2 | -1.297% | -2.013%, -0.581% | +2.397% | -0.520%, +5.314% |
| 3 | -0.343% | -0.679%, -0.007% | -0.705% | -0.238%, -1.172% |
| 4 | -0.271% | -0.635%, +0.093% | -0.674% | -0.558%, -0.790% |
| 5 | +0.866% | +1.548%, +0.183% | -1.511% | -1.897%, -1.125% |

## encode / 32 / reversiblefalse

Percent versus original; two process-phase results per order.

| Order | Candidate median | Candidate individual | Unchanged control median | Control individual |
|---|---:|---|---:|---|
| 0 | -0.663% | -1.359%, +0.032% | +0.124% | +0.421%, -0.172% |
| 1 | -3.227% | -2.903%, -3.551% | -8.982% | -1.167%, -16.798% |
| 2 | -1.358% | -1.164%, -1.552% | -1.262% | -1.739%, -0.784% |
| 3 | -0.982% | -1.008%, -0.955% | -1.225% | -0.508%, -1.942% |
| 4 | -1.007% | -1.339%, -0.675% | +0.973% | -2.425%, +4.370% |
| 5 | -0.388% | -0.787%, +0.011% | -0.470% | -0.878%, -0.061% |
