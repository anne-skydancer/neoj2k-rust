// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The C++ oracle of the Rust twin's differential tests: the C++ neo-j2k, OpenJPEG at the oracle's
//! pin and the C++ test harness, linked through the test-only shim (`shim/nj2k_rust_shim.h`), behind
//! a safe interface. Test-only: a dev-dependency, never published, and the only crate of the
//! workspace with C++ or FFI in it (the owner's rule, O4(a); CI proves `neo-j2k`'s own dependency
//! tree is free of it).
//!
//! - [`OracleBackend`]: the harness's backends (OpenJPEG, neo-j2k through its compatibility API,
//!   neo-j2k's memory API) as a [`neo_j2k_testkit::request::Backend`];
//! - [`layout_table`]: the C++ size table the twin's byte ledger mirrors;
//! - [`viewer`], [`scan_summary`], [`read_pgx`]: the C++ harness's rules, scanner and PGX reader,
//!   for the test kit's differential tests;
//! - [`kernels`]: the library's scalar row kernels;
//! - [`Codec`]: a memory codec of neo-j2k with its arena numbers and parallel thresholds.
//!
//! Every call first checks the shim's ABI version ([`init`]). The C++ never throws across the
//! boundary (the shim catches); a failure of the harness itself is a panic here, which is a test
//! failure.

use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr::NonNull;
use std::sync::OnceLock;

use neo_j2k_testkit::pgx::Pgx;
use neo_j2k_testkit::request::{Backend, Component, Mode, Outcome, Request};

mod ffi;

/// Checks the shim once: its ABI version, and that its backends were made before `main` (so that
/// making them, which writes the environment, never raced a test thread). Also sends the C++'s
/// failure reports to the console, never to a dialog.
pub fn init() {
    static CHECKED: OnceLock<()> = OnceLock::new();
    CHECKED.get_or_init(|| {
        // SAFETY: no arguments; the shim's functions are callable at any time.
        let abi = unsafe { ffi::nj2k_shim_abi_version() };
        assert_eq!(
            abi,
            ffi::ABI_VERSION,
            "the shim's ABI version is {abi}, these bindings are for {}",
            ffi::ABI_VERSION
        );
        // SAFETY: as above.
        let ready = unsafe { ffi::nj2k_shim_ready() };
        assert_eq!(ready, 1, "the shim's backends were not made before main()");
        // SAFETY: as above; it sets the C runtime's report modes of this process.
        unsafe { ffi::nj2k_shim_console_reports() };
    });
}

/// One row of the C++ size table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayoutEntry {
    /// The type's name in the twin's table (`rust/neo-j2k/src/mem/layout.rs`).
    pub name: String,
    /// `sizeof`
    pub size: u64,
    /// `alignof`
    pub align: u64,
}

/// `sizeof` and `alignof` of the types the C++ arenas hold, as this platform's compiler laid
/// them out: the decoder's, then the encoder's.
#[must_use]
pub fn layout_table() -> Vec<LayoutEntry> {
    init();
    let mut rows: *const ffi::Layout = std::ptr::null();
    // SAFETY: rows is a valid out-pointer; the shim sets it to its table, which lives as long as
    // the process, and returns the number of rows.
    let n = unsafe { ffi::nj2k_shim_layout_table(&raw mut rows) };
    assert!(!rows.is_null() && n > 0, "an empty size table");
    // SAFETY: the shim's table holds n rows at rows, unchanged for the whole run.
    let rows = unsafe { std::slice::from_raw_parts(rows, n) };
    rows.iter()
        .map(|row| {
            // SAFETY: each name is a string literal of the shim, NUL-terminated.
            let name = unsafe { CStr::from_ptr(row.name) };
            LayoutEntry {
                name: name.to_string_lossy().into_owned(),
                size: row.size,
                align: row.align,
            }
        })
        .collect()
}

/// Which of the harness's decoders a backend drives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackendKind {
    /// OpenJPEG at the oracle's pin (`oracle`).
    Oracle,
    /// neo-j2k through its OpenJPEG compatibility API (`neoj2k`).
    NeoJ2k,
    /// neo-j2k's memory API, one codec kept across requests (`neoj2k-memory`). Its viewer requests
    /// decode straight into the viewer's 8-bit image: their outcome has the geometry but no
    /// components or planes, and only [`CppHashes::u8_hash`] identifies it. Other requests go
    /// through the compatibility API, as [`BackendKind::NeoJ2k`].
    NeoJ2kMemory,
}

/// The C++ harness's own identity hashes of an outcome, for checking the test kit's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CppHashes {
    /// `Outcome::hash`
    pub hash: u64,
    /// `Outcome::u8_hash` (viewer mode)
    pub u8_hash: u64,
}

/// One of the harness's backends.
#[derive(Debug)]
pub struct OracleBackend {
    raw: NonNull<ffi::Backend>,
    name: &'static str,
}

// SAFETY: the backend is a C++ object without thread affinity; a handle is used by one thread at a
// time (`&mut self` for every call), and the shared stateless backends it may point to are safe
// for concurrent use (their decode keeps no state between requests).
unsafe impl Send for OracleBackend {}

impl OracleBackend {
    /// A backend of the kind.
    #[must_use]
    pub fn new(kind: BackendKind) -> Self {
        init();
        let (code, name) = match kind {
            BackendKind::Oracle => (ffi::BACKEND_ORACLE, "oracle"),
            BackendKind::NeoJ2k => (ffi::BACKEND_NEOJ2K, "neoj2k"),
            BackendKind::NeoJ2kMemory => (ffi::BACKEND_NEOJ2K_MEMORY, "neoj2k-memory"),
        };
        // SAFETY: a known kind; the result is a new handle or null.
        let raw = unsafe { ffi::nj2k_shim_backend_new(code) };
        Self {
            raw: NonNull::new(raw).expect("the shim could not make a backend"),
            name,
        }
    }

    /// One request, with the C++ harness's own hashes of its outcome.
    pub fn decode_with_hashes(&mut self, rq: &Request<'_>) -> (Outcome, CppHashes) {
        let request = ffi::Request {
            data: rq.data.as_ptr(),
            size: rq.data.len() as u64,
            reduce: rq.reduce,
            layers: rq.layers,
            mode: match rq.mode {
                Mode::Plain => 0,
                Mode::Strict => 1,
                Mode::Viewer => 2,
            },
            jp2: u32::from(rq.jp2),
            reduce_after_header: u32::from(rq.reduce_after_header),
            limits: u32::from(rq.limits),
            max_memory: rq.max_memory,
        };
        // SAFETY: the handle is live; request and the bytes it points to outlive the call, which
        // only reads them. The result is a new outcome or null.
        let raw = unsafe { ffi::nj2k_shim_decode(self.raw.as_ptr(), &raw const request) };
        let raw = NonNull::new(raw).expect("the C++ harness failed on a request");
        let outcome = OwnedOutcome(raw);
        outcome.read()
    }
}

impl Drop for OracleBackend {
    fn drop(&mut self) {
        // SAFETY: the handle came from nj2k_shim_backend_new and is freed once.
        unsafe { ffi::nj2k_shim_backend_free(self.raw.as_ptr()) };
    }
}

impl Backend for OracleBackend {
    fn name(&self) -> &str {
        self.name
    }
    fn decode(&mut self, rq: &Request<'_>) -> Outcome {
        self.decode_with_hashes(rq).0
    }
}

/// An outcome of the shim, freed on drop.
struct OwnedOutcome(NonNull<ffi::Outcome>);

impl Drop for OwnedOutcome {
    fn drop(&mut self) {
        // SAFETY: the outcome came from nj2k_shim_decode and is freed once.
        unsafe { ffi::nj2k_shim_outcome_free(self.0.as_ptr()) };
    }
}

impl OwnedOutcome {
    fn text(&self, which: u32) -> Vec<u8> {
        let mut len = 0u64;
        // SAFETY: the outcome is live and len a valid out-pointer.
        let p = unsafe { ffi::nj2k_shim_outcome_text(self.0.as_ptr(), which, &raw mut len) };
        if p.is_null() || len == 0 {
            return Vec::new();
        }
        let len = usize::try_from(len).expect("a text larger than memory");
        // SAFETY: p points to len bytes owned by the outcome, which outlives this copy.
        unsafe { std::slice::from_raw_parts(p, len) }.to_vec()
    }

    fn read(&self) -> (Outcome, CppHashes) {
        let mut info = ffi::OutcomeInfo::default();
        // SAFETY: the outcome is live and info a valid out-pointer.
        unsafe { ffi::nj2k_shim_outcome_info(self.0.as_ptr(), &raw mut info) };
        let mut o = Outcome {
            ok: info.ok != 0,
            stage: self.text(ffi::TEXT_STAGE),
            x0: info.x0,
            y0: info.y0,
            x1: info.x1,
            y1: info.y1,
            numcomps: info.numcomps,
            color_space: info.color_space,
            warnings: info.warnings,
            errors: info.errors,
            last_error: self.text(ffi::TEXT_LAST_ERROR),
            warning_kinds: self.text(ffi::TEXT_WARNING_KINDS),
            ..Outcome::default()
        };
        for c in 0u32.. {
            let mut k = ffi::Component::default();
            let mut samples: *const i32 = std::ptr::null();
            let mut count = 0u64;
            // SAFETY: the outcome is live and the out-pointers valid; 0 past the last component.
            let more = unsafe {
                ffi::nj2k_shim_outcome_component(self.0.as_ptr(), c, &raw mut k, &raw mut samples, &raw mut count)
            };
            if more == 0 {
                break;
            }
            o.comps.push(Component {
                w: k.w,
                h: k.h,
                dx: k.dx,
                dy: k.dy,
                x0: k.x0,
                y0: k.y0,
                prec: k.prec,
                sgnd: k.sgnd,
                factor: k.factor,
                resno_decoded: k.resno_decoded,
            });
            if !samples.is_null() && count > 0 {
                let count = usize::try_from(count).expect("a plane larger than memory");
                // SAFETY: samples points to count samples owned by the outcome, which outlives
                // this copy.
                o.planes
                    .push(unsafe { std::slice::from_raw_parts(samples, count) }.to_vec());
            } else if o.ok {
                o.planes.push(Vec::new());
            }
        }
        (
            o,
            CppHashes {
                hash: info.hash,
                u8_hash: info.u8_hash,
            },
        )
    }
}

/// The C++ harness's viewer rules (`tests/common/viewer.cpp`).
pub mod viewer {
    use crate::{ffi, init};

    /// `data_size_j2c`
    #[must_use]
    pub fn data_size_j2c(w: i32, h: i32, discard: i32) -> i64 {
        init();
        // SAFETY: plain values in and out.
        unsafe { ffi::nj2k_shim_viewer_data_size_j2c(w, h, discard) }
    }
    /// `grid_side`
    #[must_use]
    pub fn grid_side(tiles: u64, tile_size: u32) -> i32 {
        init();
        // SAFETY: plain values in and out.
        unsafe { ffi::nj2k_shim_viewer_grid_side(tiles, tile_size) }
    }
    /// `components_supported`
    #[must_use]
    pub fn components_supported(components: u64) -> bool {
        init();
        // SAFETY: plain values in and out.
        unsafe { ffi::nj2k_shim_viewer_components_supported(components) != 0 }
    }
    /// `reduce_factor`
    #[must_use]
    pub fn reduce_factor(
        requested_discard: i32,
        resolutions: i32,
        width: i32,
        height: i32,
        max_image_size: i32,
    ) -> i32 {
        init();
        // SAFETY: plain values in and out.
        unsafe { ffi::nj2k_shim_viewer_reduce_factor(requested_discard, resolutions, width, height, max_image_size) }
    }
    /// `decode_allowed`
    #[must_use]
    pub fn decode_allowed(width: u64, height: u64, components: u64, reduce: u32) -> bool {
        init();
        // SAFETY: plain values in and out.
        unsafe { ffi::nj2k_shim_viewer_decode_allowed(width, height, components, reduce) != 0 }
    }
}

/// The C++ scanner's summary of a codestream (`neo_j2k_testkit::scan::summary` writes the same).
#[must_use]
pub fn scan_summary(bytes: &[u8]) -> String {
    init();
    // SAFETY: bytes is valid for its length; a null output with cap 0 asks for the length only.
    let n = unsafe { ffi::nj2k_shim_scan_summary(bytes.as_ptr(), bytes.len(), std::ptr::null_mut(), 0) };
    let mut out = vec![0u8; n];
    // SAFETY: as above, out has room for the n bytes the summary takes.
    let m = unsafe { ffi::nj2k_shim_scan_summary(bytes.as_ptr(), bytes.len(), out.as_mut_ptr().cast(), out.len()) };
    assert_eq!(m, n, "the C++ scan's summary changed between two calls");
    String::from_utf8(out).expect("the summary is ASCII")
}

/// The C++ PGX reader (`neo_j2k_testkit::pgx::read_pgx` reads the same); `Err` with its reason.
pub fn read_pgx(path: &Path) -> Result<Pgx, String> {
    init();
    let path = CString::new(path.to_str().expect("a UTF-8 path")).expect("a path without NUL");
    let mut why = [0u8; 256];
    // SAFETY: path is NUL-terminated; why has room for why.len() bytes, which the shim
    // NUL-terminates. The result is a new image or null.
    let raw = unsafe { ffi::nj2k_shim_read_pgx(path.as_ptr(), why.as_mut_ptr().cast(), why.len()) };
    let Some(raw) = NonNull::new(raw) else {
        let end = why.iter().position(|&b| b == 0).unwrap_or(why.len());
        return Err(String::from_utf8_lossy(&why[..end]).into_owned());
    };
    let (mut w, mut h, mut depth, mut signed) = (0i32, 0i32, 0i32, 0u32);
    let mut samples: *const i32 = std::ptr::null();
    let mut count = 0u64;
    // SAFETY: the image is live and every out-pointer valid.
    unsafe {
        ffi::nj2k_shim_pgx_info(
            raw.as_ptr(),
            &raw mut w,
            &raw mut h,
            &raw mut depth,
            &raw mut signed,
            &raw mut samples,
            &raw mut count,
        );
    }
    let data = if samples.is_null() || count == 0 {
        Vec::new()
    } else {
        let count = usize::try_from(count).expect("an image larger than memory");
        // SAFETY: samples points to count samples owned by the image, freed only below.
        unsafe { std::slice::from_raw_parts(samples, count) }.to_vec()
    };
    // SAFETY: the image came from nj2k_shim_read_pgx and is freed once.
    unsafe { ffi::nj2k_shim_pgx_free(raw.as_ptr()) };
    Ok(Pgx {
        width: w,
        height: h,
        depth,
        is_signed: signed != 0,
        samples: data,
    })
}

/// The library's scalar row kernels, as the library compiled them.
pub mod kernels {
    use crate::{ffi, init};

    /// `dc_shift_53_scalar` on a row, in place.
    pub fn dc_shift_53_scalar(row: &mut [i32], shift: i32, lo: i32, hi: i32) {
        init();
        // SAFETY: row is valid for reading and writing its length.
        unsafe { ffi::nj2k_shim_dc_shift_53_scalar(row.as_mut_ptr(), row.len(), shift, lo, hi) }
    }
    /// `dc_shift_97_scalar` on a row of float bit patterns, in place.
    pub fn dc_shift_97_scalar(row: &mut [i32], shift: i32, lo: i32, hi: i32) {
        init();
        // SAFETY: row is valid for reading and writing its length.
        unsafe { ffi::nj2k_shim_dc_shift_97_scalar(row.as_mut_ptr(), row.len(), shift, lo, hi) }
    }

    fn colour(
        f: unsafe extern "C" fn(*const i32, *const i32, *const i32, *mut i32, *mut i32, *mut i32, usize),
        c: [&[i32]; 3],
    ) -> [Vec<i32>; 3] {
        init();
        let n = c[0].len();
        assert!(c[1].len() == n && c[2].len() == n, "three rows of one length");
        let mut o = [vec![0; n], vec![0; n], vec![0; n]];
        let [o0, o1, o2] = &mut o;
        // SAFETY: three input rows and three distinct output rows of n samples each.
        unsafe {
            f(
                c[0].as_ptr(),
                c[1].as_ptr(),
                c[2].as_ptr(),
                o0.as_mut_ptr(),
                o1.as_mut_ptr(),
                o2.as_mut_ptr(),
                n,
            )
        };
        o
    }
    /// `rct_scalar`: the reversible colour transform's inverse, out of place.
    #[must_use]
    pub fn rct_scalar(c: [&[i32]; 3]) -> [Vec<i32>; 3] {
        colour(ffi::nj2k_shim_rct_scalar, c)
    }
    /// `ict_scalar`: the irreversible colour transform's inverse on float bit patterns, out of
    /// place.
    #[must_use]
    pub fn ict_scalar(c: [&[i32]; 3]) -> [Vec<i32>; 3] {
        colour(ffi::nj2k_shim_ict_scalar, c)
    }
}

/// A codec's container format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// A codestream (`NJ2K_CODEC_J2K`).
    J2k,
    /// A JP2 file (`NJ2K_CODEC_JP2`).
    Jp2,
}

/// The C++ arenas' numbers of a codec (`nj2k_shim_ledger_t`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ledger {
    /// The header arena's reserved bytes.
    pub header_reserved: u64,
    /// The tile-part arena's.
    pub tile_reserved: u64,
    /// The work arena's.
    pub work_reserved: u64,
    /// The work arena's high-water mark (0 once a decode's trim reset it).
    pub work_high_water: u64,
    /// `Decoder::kept_bytes()`.
    pub kept_bytes: u64,
}

/// A non-strict memory decoder of the C++ neo-j2k.
#[derive(Debug)]
pub struct Codec(NonNull<ffi::Codec>);

// SAFETY: a C++ codec serves one thread at a time (the library's rule); `&mut self` on every call
// gives that, and the codec has no thread affinity.
unsafe impl Send for Codec {}

impl Codec {
    /// A decoder at a reduce factor, with `max_memory` as its only limit when not 0.
    #[must_use]
    pub fn new(format: Format, reduce: u32, max_memory: u64) -> Option<Self> {
        init();
        let f = match format {
            Format::J2k => ffi::CODEC_J2K,
            Format::Jp2 => ffi::CODEC_JP2,
        };
        // SAFETY: plain values; the result is a new codec or null.
        NonNull::new(unsafe { ffi::nj2k_shim_codec_new(f, reduce, max_memory) }).map(Self)
    }

    /// New limits (`max_memory` alone); false when the codec refuses them now.
    pub fn set_max_memory(&mut self, max_memory: u64) -> bool {
        // SAFETY: the codec is live.
        unsafe { ffi::nj2k_shim_codec_set_max_memory(self.0.as_ptr(), max_memory) != 0 }
    }

    /// The header from memory, then the decode into planar `int32` planes (each component `w x h`,
    /// one after the other): the status (`NJ2K_OK`, `NJ2K_TRUNCATED` or an error) and the samples.
    pub fn decode_i32(&mut self, data: &[u8]) -> (i32, Vec<i32>) {
        let mut need = 0u64;
        // SAFETY: the codec is live, data valid for its length; with no output (cap 0) the shim
        // reads the header and gives the samples needed.
        let st = unsafe {
            ffi::nj2k_shim_codec_decode_i32(
                self.0.as_ptr(),
                data.as_ptr(),
                data.len(),
                std::ptr::null_mut(),
                0,
                &raw mut need,
            )
        };
        if need == 0 {
            return (st, Vec::new());
        }
        let mut out = vec![0i32; usize::try_from(need).expect("an image larger than memory")];
        let mut count = 0u64;
        // SAFETY: as above, out has room for the need samples the header gives.
        let st = unsafe {
            ffi::nj2k_shim_codec_decode_i32(
                self.0.as_ptr(),
                data.as_ptr(),
                data.len(),
                out.as_mut_ptr(),
                need,
                &raw mut count,
            )
        };
        assert_eq!(count, need, "the header changed between two reads of the same bytes");
        (st, out)
    }

    /// The arenas' numbers now.
    #[must_use]
    pub fn ledger(&mut self) -> Ledger {
        let mut l = ffi::Ledger::default();
        // SAFETY: the codec is live and l a valid out-pointer.
        let ok = unsafe { ffi::nj2k_shim_ledger(self.0.as_ptr(), &raw mut l) };
        assert_eq!(ok, 1);
        Ledger {
            header_reserved: l.header_reserved,
            tile_reserved: l.tile_reserved,
            work_reserved: l.work_reserved,
            work_high_water: l.work_high_water,
            kept_bytes: l.kept_bytes,
        }
    }

    /// The sizes the decoder splits work from (`set_parallel_thresholds`); 0s force every stage
    /// through an executor. Without an executor they change nothing.
    pub fn set_parallel_thresholds(&mut self, tile_samples: u64, level_area: u64, output: u64, clear: u64) -> bool {
        // SAFETY: the codec is live.
        unsafe { ffi::nj2k_shim_set_parallel_thresholds(self.0.as_ptr(), tile_samples, level_area, output, clear) != 0 }
    }
}

impl Drop for Codec {
    fn drop(&mut self) {
        // SAFETY: the codec came from nj2k_shim_codec_new and is freed once.
        unsafe { ffi::nj2k_shim_codec_free(self.0.as_ptr()) };
    }
}
