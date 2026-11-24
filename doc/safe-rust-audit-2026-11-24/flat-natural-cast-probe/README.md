# Safe natural cast lowering in the corrected flat loop

External control experiment only; production untouched. Both candidate variants retain the proven flat per-pixel three-reference colour loop. One performs f32.max/min with static±2^30 bounds before round_ties_even and as-i32; the other rounds first then performs the static clamps. Final integer Finish bounds/shift preserve the supported precision contract.

Both assembly listings show packed ICT and rounding followed by per-lane extract/compare/branch/reinsert saturation code. Neither has packed vcvttps2dq in its candidate hot loop; packed conversion remains in original reference. The static clamp does not eliminate saturating cast lowering on the pinned compiler. No timing or broad correctness claim is made; the inferior lowering was sufficient to reject further production work on these forms.
