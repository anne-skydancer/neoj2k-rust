# Final helper mapping

The disassembler prints the nearest exported symbol as its heading, which is not the private function being investigated. PDB records identify the actual ranges.

- Original reference executable: `renormalize_with_bytes`, section1 offset321056decimal,283 bytes, VA0x14004F620..0x14004F73B. Its frozen executable/PDB hashes are in `reference-hashes.txt`; the binary/PDB stay in TEMP, not this repository.
- Packed-probe executable: old aggregate helper also remains, section1 offset363520decimal,283 bytes,64 bytes of saved nonvolatile registers.
- Packed-probe scalar helper: section1 offset364304decimal,202 bytes, VA0x140059F10..0x140059FDA,40 bytes of saved nonvolatile registers.

Current internalRustABI lowering uses RCX as hiddenresultpointer, RDX as pointer to12-byte inputRegisters, R8 asSourcepointer, R9 asshiftcount. Candidate uses c inECX,bits inEDX,Source inR8,total inR9 and packs result intoRAX. These are observations of Rust1.100.0 x86_64-pc-windows-msvc finalbuilds, not permanentRustABI guarantees. Inspect PDB excerpts and machineinstructions rather than trusting the nearestexport label.
