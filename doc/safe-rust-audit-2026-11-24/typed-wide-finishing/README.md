# Typed wide finishing experiment

The public paired driver uses the original d13082f reference, current candidate, and an otherwise-identical three-operation safe control. The inherited raw header mentions the earlier DWT experiment; the actual Cargo manifest selects safe-threeop-control. Percent columns are candidate/control, candidate/original, control/original (paired medians may differ from ratios of separately reported absolute medians).

RGB typed caps: full native/C++ suite 108 passed, 7 ignored. Three pinned precision processes show large p31 improving 2.04–2.44% against the three-operation control; large p24 improves 0.99–1.86%. Small rows have regressions/variance, so this is not performance acceptance. These public runs did not yet include the new unary path.

Unary typed converter micro: 1024 wide values approximately 86–87 ns versus f64 108 ns; all driver equality checks pass. Combined RGB+unary production candidate is undergoing independent native/reference and public checks. No unsafe/dependency added.

Combined unary+RGB follow-up: full native/C++108 passed, Clippy passed, six precision and six tail rows in three processes all exact. The earlier large-image RGB gain did not survive this combined build: large p31 −0.61..+0.39% vs three-op control, large p24 −0.42..+1.14%; small64 remains slower. Candidate is for cross-platform assessment, not accepted performance.
