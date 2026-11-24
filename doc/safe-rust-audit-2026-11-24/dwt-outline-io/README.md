# Dense DWT I/O call-boundary experiment

Candidate changes only fill_dense and scatter_dense from inline(always) to inline(never). Dense scratch layout, lifting math, edge order, scheduling and budget are unchanged. This differs from rejected outlining of the complete row kernel.

Realistic-stride micro: at stride128, width32 current82ns/candidate75/original71; width64 current137/candidate123/original126; width128 current260/candidate232/original252. All padding/sample equality checks pass. Full native/C++ library108 pass,7ignored. Whole-codec paired assessment pending; not accepted yet.

Three paired public processes: largep31 improves0.91–1.43% versus identical safe control; normal64whole improves0.26–1.67%; largep24 improves0.08–0.69%. Some small rows vary and encode32 lossless worsens1.06–2.02%, lossy0.39–4.02%, so overall performance not accepted.

Actual executable encoder inspection: corresponding Encoder::encode, gather_samples, tier1::encode_all_passes functions have identical1482/639/1079 normalized instructions and equal byte sizes, after replacing branch/call relocation targets and RIP-relative displacements. Raw and normalized listings are preserved. This supports placement/cache sensitivity; it does not establish complete pipeline identity or erase measured gaps.

Attribution: gather-only outlining is1–4% slower; scatter-only improves about11–12% and slightly outperforms both outlined. At stride128 width64 current137/scatter121ns/original130; width128 current259/scatter228/original250. All checked samples/padding exact. The smaller scatter-only candidate is undergoing its own public comparison; both-outlined variant is not accepted.

Scatter-only follow-up:108 native/C++ tests pass; three public processes all exact/status. Largep31 consistently improves0.87–1.37% versus the unchanged safe control. Largep24 is mixed(+0.33/−1.00/−0.70%); small/packed rows are unstable and encoder32 remains slower1.1–3.7%. Source affects only one inline attribute. Windows assessment and final full matrix pending.
