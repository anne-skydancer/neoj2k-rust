# BYTEIN constant-shift branch experiment (rejected)

A separate instrumented clone counted real BYTEIN events on five fixtures. Non-FF current byte accounts for98.22% at64RGB,98.92% at128RGBA,99.48% at large1024. Precision-patched payloads have identical counts. These are histograms, not speed measurements. No production instrumentation was added.

Original final assembly retains SETcc bit-count construction, a subtraction forming16-bits and variable SHLX for the next byte. Candidate splits current!=FF first and returns constant8bits/next<<8; stuffed data returns7bits/next<<9, markers preserve original no-consumption semantics. Existing7 MQ model/C++ tests pass (nightly443million enumeration ignored); actual assembly emits constant SHL8/9. Renormalize grows212→236bytes.

Direct public paired six processes/39 rows all exact/status, with precision rows additionally C++ matched. Nevertheless largep31 worsens2.07/2.36/2.63% versus prior safe control, largep24+0.49/+1.10/+1.13%; packed3+1.49..2.43%. Fewer apparent common-path instructions did not improve the codec. Candidate rejected; main Linux and Windows source unchanged. No broad MQ rewrite or aggregate-ABI change was made.

Branchless assembly control: original marker/count flow with select_unpredictable(stuffed,next<<9,next<<8) canonicalizes back to variable SHLX/count arithmetic on the pinned compiler. No meaningful lowering delta; rejected without a further whole build or timing run. Minimal source/assembly preserved. MQ exploration stopped after this bounded control.
