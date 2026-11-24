# Bounded general colour finishing probes

Intermediate experiments only. None of these alternatives was applied to production. The f64 source is the current production control snapshot; the reference/control modules in each driver point at the unchanged original crate in the Linux audit sandbox. Drivers require those absolute sandbox paths to reproduce.

The drivers use runtime-blackboxed signed precision-31 finishes, exact sample checks, widths 32/128/1024, three input magnitudes, three rotating implementations and twelve rounds of 10,000 calls. Built with rustc 1.100.0, edition2024, -O and target-cpu=x86-64-v3; timing process pinned to CPU7. Runs are diagnostic, not acceptance evidence across the codec.

At width1024, current safe f64 measured roughly471ns versus original274ns in its run. Four-wide f64 took797ns, static bounded float cast1929ns, scalar-map split-bias906ns, and flat exponent conversion5052–5140ns in their respective runs. These alternatives are rejected. Uniform RGB finish specialization measured458ns versus original269ns, a small possible improvement over the earlier f64 run, requiring a direct same-process comparison before consideration. It remains experimental.

Every recorded run completed in under three seconds including compilation where performed. No probe replaces the public precision-24/31 codec comparisons or final Miri validation.
