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
//! - [`kernels`]: the library's scalar row kernels and inverse wavelet transform;
//! - [`Codec`]: a memory codec of neo-j2k with header facts, decoded dimensions, arena numbers and parallel thresholds.
//! - [`encode_u8`]: the C++ encoder's codestream from eight-bit samples and explicit encoder settings.
//!
//! Every call first checks the shim's ABI version ([`init`]). The C++ never throws across the
//! boundary (the shim catches); a failure of the harness itself is a panic here, which is a test
//! failure.

use std::ffi::{CStr, CString};
use std::num::NonZeroUsize;
use std::path::Path;
use std::ptr::NonNull;
use std::sync::OnceLock;

use neo_j2k_testkit::pgx::Pgx;
use neo_j2k_testkit::request::{Backend, Component, Mode, Outcome, Request};

mod ffi;
mod pool;

pub use pool::BenchmarkPool;

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
    /// The type's name in the twin's table (`neo-j2k/src/mem/layout.rs`).
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
            o.comps.push(Component::from(k));
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

impl From<ffi::Component> for Component {
    fn from(raw: ffi::Component) -> Self {
        Self {
            w: raw.w,
            h: raw.h,
            dx: raw.dx,
            dy: raw.dy,
            x0: raw.x0,
            y0: raw.y0,
            prec: raw.prec,
            sgnd: raw.sgnd,
            factor: raw.factor,
            resno_decoded: raw.resno_decoded,
        }
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

/// The library's scalar row kernels and inverse wavelet transform, as the library compiled them.
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

    /// The bounds of one resolution of a tile-component in that resolution's coordinates.
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct WaveletRect {
        pub x0: u32,
        pub y0: u32,
        pub x1: u32,
        pub y1: u32,
    }

    /// Inverse wavelet on the given tile-component. Resolutions go from coarsest to finest.
    /// The plane includes the full `stride` of each row, and holds float bits for 9/7.
    /// Returns false if the shim refuses the geometry or cannot allocate scratch.
    #[must_use]
    pub fn dwt_decode(plane: &mut [i32], stride: u32, resolutions: &[WaveletRect], reversible: bool) -> bool {
        init();
        // SAFETY: both slices are valid for the call. Before the transform the shim checks the
        // chain of resolutions, every coordinate and the complete strided plane size.
        unsafe {
            ffi::nj2k_shim_dwt_decode(
                plane.as_mut_ptr(),
                plane.len(),
                stride,
                resolutions.as_ptr(),
                resolutions.len(),
                u32::from(reversible),
            ) != 0
        }
    }
}

/// The inputs of one neo-j2k C++ encoder call. Samples are interleaved with an optional row stride.
#[derive(Clone, Copy, Debug)]
pub struct EncodeRequest<'a> {
    pub pixels: &'a [u8],
    pub row_stride: usize,
    pub width: u32,
    pub height: u32,
    pub components: u32,
    pub flip_vertical: bool,
    pub reversible: bool,
    pub resolutions: u32,
    pub mct: bool,
    pub max_codestream_size: u32,
    pub layer_ratios: &'a [f32],
    pub comment: Option<&'a [u8]>,
}

struct Codestream(NonNull<ffi::Codestream>);

impl Codestream {
    fn bytes(&self) -> Vec<u8> {
        let mut len = 0;
        // SAFETY: the codestream handle is live and len is a writable local.
        let bytes = unsafe { ffi::nj2k_shim_codestream_bytes(self.0.as_ptr(), &raw mut len) };
        if len == 0 {
            return Vec::new();
        }
        // SAFETY: the C++ codestream owns len initialized bytes for as long as the handle lives.
        unsafe { std::slice::from_raw_parts(bytes, len) }.to_vec()
    }
}

impl Drop for Codestream {
    fn drop(&mut self) {
        // SAFETY: the handle was returned by the shim and is freed here exactly once.
        unsafe { ffi::nj2k_shim_codestream_free(self.0.as_ptr()) };
    }
}

/// Encode into a memory output stream. Errors are the C++ encoder's status codes.
pub fn encode_u8(request: &EncodeRequest<'_>) -> Result<Vec<u8>, i32> {
    encode_request(request, |raw, out| {
        // SAFETY: encode_request supplies the live input and a writable output handle.
        unsafe { ffi::nj2k_shim_encode_u8(raw, out) }
    })
}

/// The parameters written back by a successful encoder setup.
#[derive(Clone, Debug)]
pub struct EncodeUpdates {
    pub max_codestream_size: u32,
    pub layer_ratios: Vec<f32>,
}

/// Encodes and returns the settings written back by the C++ encoder's setup call, if it succeeded.
pub fn encode_u8_with_updates(request: &EncodeRequest<'_>) -> (Result<Vec<u8>, i32>, Option<EncodeUpdates>) {
    let mut updates = ffi::EncodeUpdates {
        max_codestream_size: 0,
        layer_count: 0,
        ratios: [0.0; 100],
    };
    let output = encode_request(request, |raw, out| {
        // SAFETY: encode_request supplies the live input and writable output handle. The update
        // record is a local with the C struct's repr(C) layout and stays live through the call.
        unsafe { ffi::nj2k_shim_encode_u8_with_updates(raw, &raw mut updates, out) }
    });
    let updates = (updates.layer_count != 0).then(|| EncodeUpdates {
        max_codestream_size: updates.max_codestream_size,
        layer_ratios: updates.ratios[..updates.layer_count as usize].to_vec(),
    });
    (output, updates)
}

/// How the C++ stream callback accepts bytes during an encoder trace.
#[derive(Clone, Copy, Debug)]
pub struct EncodeTraceOptions<'a> {
    pub buffer_size: NonZeroUsize,
    pub max_write: usize,
    pub stop_at: usize,
    pub fail_with_error: bool,
    pub extra_options: &'a [String],
}

/// The C++ encoder's status and callback observations, including after a refused write.
#[derive(Debug, PartialEq, Eq)]
pub struct EncodeTrace {
    pub status: i32,
    pub bytes: Vec<u8>,
    pub offers: Vec<usize>,
    pub diagnostics: Vec<HeaderDiagnostic>,
}

/// Encodes to a traced C++ stream using the same setup as [`encode_u8`].
pub fn encode_u8_trace(request: &EncodeRequest<'_>, options: EncodeTraceOptions<'_>) -> EncodeTrace {
    init();
    let request = raw_encode_request(request);
    let strings: Vec<_> = options
        .extra_options
        .iter()
        .map(|option| CString::new(option.split('\0').next().unwrap_or_default()).unwrap())
        .collect();
    let mut pointers: Vec<_> = strings.iter().map(|option| option.as_ptr()).collect();
    pointers.push(std::ptr::null());
    let trace = ffi::EncodeTraceRequest {
        buffer_size: options.buffer_size.get(),
        max_write: options.max_write,
        stop_at: options.stop_at,
        fail_with_error: u32::from(options.fail_with_error),
        extra_options: pointers.as_ptr(),
    };
    let mut out = std::ptr::null_mut();
    // SAFETY: request spans and the null-terminated extra-options list remain valid until return.
    let status = unsafe { ffi::nj2k_shim_encode_u8_trace(&raw const request, &raw const trace, &raw mut out) };
    let stream = Codestream(NonNull::new(out).expect("the C++ harness did not return an encode trace"));
    let mut count = 0;
    // SAFETY: the codestream handle is live and count points to a writable local.
    let offers = unsafe { ffi::nj2k_shim_codestream_offers(stream.0.as_ptr(), &raw mut count) };
    let offers = if count == 0 {
        Vec::new()
    } else {
        // SAFETY: the C++ codestream owns count initialized offers until the handle is freed.
        unsafe { std::slice::from_raw_parts(offers, count) }.to_vec()
    };
    let diagnostics = read_diagnostics(|index, severity, text, len| {
        // SAFETY: the snapshot is live and the out-pointers are valid, with zero past the last message.
        unsafe { ffi::nj2k_shim_codestream_diagnostic(stream.0.as_ptr(), index, severity, text, len) }
    });
    EncodeTrace {
        status,
        bytes: stream.bytes(),
        offers,
        diagnostics,
    }
}

fn encode_request(
    request: &EncodeRequest<'_>,
    encode: impl FnOnce(*const ffi::EncodeRequest, *mut *mut ffi::Codestream) -> i32,
) -> Result<Vec<u8>, i32> {
    init();
    let raw = raw_encode_request(request);
    get_codestream(|out| encode(&raw const raw, out))
}

fn raw_encode_request(request: &EncodeRequest<'_>) -> ffi::EncodeRequest {
    let (comment, comment_len) = request.comment.map_or((std::ptr::null(), 0), |c| (c.as_ptr(), c.len()));
    ffi::EncodeRequest {
        pixels: request.pixels.as_ptr(),
        size: request.pixels.len(),
        stride: request.row_stride,
        width: request.width,
        height: request.height,
        components: request.components,
        flip_vertical: u32::from(request.flip_vertical),
        reversible: u32::from(request.reversible),
        resolutions: request.resolutions,
        mct: u32::from(request.mct),
        max_codestream_size: request.max_codestream_size,
        rates: request.layer_ratios.as_ptr(),
        rate_count: request.layer_ratios.len(),
        comment,
        comment_len,
    }
}

/// Generates a synthetic JPEG 2000 codestream with the pinned OpenJPEG encoder.
///
/// The seed and case index select the sample data and bounded coding parameters.
pub fn generate_codestream(seed: u32, index: u32) -> Result<Vec<u8>, i32> {
    init();
    get_codestream(|out| {
        // SAFETY: get_codestream supplies a writable output handle.
        unsafe { ffi::nj2k_shim_generate_codestream(seed, index, out) }
    })
}

fn get_codestream(encode: impl FnOnce(*mut *mut ffi::Codestream) -> i32) -> Result<Vec<u8>, i32> {
    let mut out = std::ptr::null_mut();
    let status = encode(&raw mut out);
    if status != 0 {
        return Err(status);
    }
    Ok(Codestream(NonNull::new(out).expect("the C++ encoder returned its codestream")).bytes())
}

/// A codec's container format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// A codestream (`NJ2K_CODEC_J2K`).
    J2k,
    /// A JP2 file (`NJ2K_CODEC_JP2`).
    Jp2,
}

/// Reduce policies of the C++ decoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ReducePolicy {
    /// Refuse a reduce factor above the coarsest resolution.
    Exact = 0,
    /// Lower the factor to the coarsest resolution shared by all components.
    Clamp = 1,
}

/// Resource limits of the C++ decoder. An individual zero leaves the resource unbounded.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Limits {
    /// Tiles in the reference grid.
    pub max_tiles: u32,
    /// Components in the image.
    pub max_components: u32,
    /// Quality layers in the main header or a tile.
    pub max_layers: u32,
    /// Width or height of a decoded component.
    pub max_decoded_side: u32,
    /// Bytes available to the decoder's allocations.
    pub max_memory: u64,
}

/// Header facts from the C++ memory API and the result of its separate dimensions check.
#[derive(Debug)]
pub struct Header {
    /// Image bounds `[x0, y0, x1, y1]` on the reference grid.
    pub bounds: [u32; 4],
    /// Tile grid `[x0, y0, tile width, tile height, columns, rows]`.
    pub tiles: [u32; 6],
    /// The quality layer count in the main header.
    pub layers: u32,
    /// The fewest resolutions any component has in the main header.
    pub min_resolutions: u32,
    /// Progression Order in the main header (`NJ2K_PROG_ORDER`).
    pub progression: u32,
    /// `NJ2K_COLOR_SPACE` of the image.
    pub color_space: i32,
    /// Components as reported directly by `nj2k_read_header_memory`.
    pub components: Vec<Component>,
    /// Status of a reduce change requested after the header read, if requested.
    pub reduce_status: Option<i32>,
    /// Components after `nj2k_get_decoded_dimensions`, or its exact failure status.
    pub dimensions: Result<Vec<Component>, i32>,
}

/// A diagnostic from the C++ codec callback.
#[derive(Debug, PartialEq, Eq)]
pub enum HeaderDiagnostic {
    /// An informational message.
    Info(Vec<u8>),
    /// A recoverable warning.
    Warning(Vec<u8>),
    /// An error message.
    Error(Vec<u8>),
}

struct OwnedHeader(NonNull<ffi::Header>);

impl OwnedHeader {
    fn components(&self, count: u32, sized: bool) -> Vec<Component> {
        let count = usize::try_from(count).expect("component count fits in memory");
        // SAFETY: the snapshot is live. When sized is true, the dimensions check succeeded.
        let components = unsafe { ffi::nj2k_shim_header_components(self.0.as_ptr(), u32::from(sized)) };
        assert!(!components.is_null(), "an empty header component array");
        // SAFETY: the shim filled count entries for this snapshot and it outlives the copy.
        unsafe { std::slice::from_raw_parts(components, count) }
            .iter()
            .copied()
            .map(Component::from)
            .collect()
    }

    fn diagnostics(&self) -> Vec<HeaderDiagnostic> {
        read_diagnostics(|index, severity, text, len| {
            // SAFETY: the snapshot is live and the out-pointers are valid, with zero past the last message.
            unsafe { ffi::nj2k_shim_header_diagnostic(self.0.as_ptr(), index, severity, text, len) }
        })
    }
}

fn read_diagnostics(mut read: impl FnMut(usize, &mut u32, &mut *const u8, &mut usize) -> u32) -> Vec<HeaderDiagnostic> {
    let mut diagnostics = Vec::new();
    for index in 0.. {
        let mut severity = 0;
        let mut text = std::ptr::null();
        let mut len = 0;
        if read(index, &mut severity, &mut text, &mut len) == 0 {
            break;
        }
        // SAFETY: the shim returns len text bytes owned by the live snapshot.
        let bytes = unsafe { std::slice::from_raw_parts(text, len) }.to_vec();
        diagnostics.push(match severity {
            0 => HeaderDiagnostic::Info(bytes),
            1 => HeaderDiagnostic::Warning(bytes),
            2 => HeaderDiagnostic::Error(bytes),
            _ => panic!("unexpected C++ diagnostic severity {severity}"),
        });
    }
    diagnostics
}

impl Drop for OwnedHeader {
    fn drop(&mut self) {
        // SAFETY: the snapshot came from nj2k_shim_codec_header and is freed once.
        unsafe { ffi::nj2k_shim_header_free(self.0.as_ptr()) };
    }
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

/// A memory decoder of the C++ neo-j2k.
#[derive(Debug)]
pub struct Codec(NonNull<ffi::Codec>);

// SAFETY: a C++ codec serves one thread at a time (the library's rule); `&mut self` on every call
// gives that, and the codec has no thread affinity.
unsafe impl Send for Codec {}

impl Codec {
    /// A decoder at a reduce factor, with `max_memory` as its only limit when not 0.
    #[must_use]
    pub fn new(format: Format, reduce: u32, max_memory: u64) -> Option<Self> {
        Self::with_limits(
            format,
            reduce,
            ReducePolicy::Exact,
            Limits {
                max_memory,
                ..Limits::default()
            },
        )
    }

    /// A decoder with explicit reduce policy and resource limits.
    #[must_use]
    pub fn with_limits(format: Format, reduce: u32, reduce_policy: ReducePolicy, limits: Limits) -> Option<Self> {
        Self::with_options(format, reduce, reduce_policy, limits, 0, false)
    }

    /// A memory decoder with explicit reduce, resource, layer and strict-mode options.
    #[must_use]
    pub fn with_options(
        format: Format,
        reduce: u32,
        reduce_policy: ReducePolicy,
        limits: Limits,
        max_layers: u32,
        strict: bool,
    ) -> Option<Self> {
        init();
        let f = match format {
            Format::J2k => ffi::CODEC_J2K,
            Format::Jp2 => ffi::CODEC_JP2,
        };
        // SAFETY: limits is valid for the call and copied by the shim. The result is a new codec or null.
        NonNull::new(unsafe {
            ffi::nj2k_shim_codec_new(
                f,
                reduce,
                reduce_policy as u32,
                &raw const limits,
                max_layers,
                u32::from(strict),
            )
        })
        .map(Self)
    }

    /// New limits (`max_memory` alone); false when the codec refuses them now.
    pub fn set_max_memory(&mut self, max_memory: u64) -> bool {
        // SAFETY: the codec is live.
        unsafe { ffi::nj2k_shim_codec_set_max_memory(self.0.as_ptr(), max_memory) != 0 }
    }

    /// Set the requested reduce factor on the existing decoder and return its status.
    pub fn set_reduce(&mut self, reduce: u32) -> i32 {
        // SAFETY: the codec is live and the factor is passed by value.
        unsafe { ffi::nj2k_shim_codec_set_reduce(self.0.as_ptr(), reduce) }
    }

    /// Change the capacity the C++ codec can retain after a decode.
    pub fn set_retained_bytes(&mut self, bytes: usize) -> bool {
        // SAFETY: the codec is live and the byte limit is passed by value.
        unsafe { ffi::nj2k_shim_codec_set_retained_bytes(self.0.as_ptr(), bytes) != 0 }
    }

    /// Read a memory header and check decoded dimensions without decoding samples.
    /// Returns the exact C++ status if the header read fails.
    pub fn read_header(&mut self, data: &[u8]) -> Result<Header, i32> {
        self.read_header_request(data, None, false).0
    }

    /// Read a header, set a new reduce factor, then check decoded dimensions on the original image.
    pub fn read_header_then_reduce(&mut self, data: &[u8], reduce: u32) -> Result<Header, i32> {
        self.read_header_request(data, Some(reduce), false).0
    }

    /// Read a memory header, select codestream components, and check the resulting dimensions.
    pub fn decoded_dimensions_for(&mut self, data: &[u8], indices: &[u32]) -> Result<Vec<Component>, i32> {
        let count = u32::try_from(indices.len()).expect("component count fits in u32");
        let mut numcomps = 0;
        let mut raw = std::ptr::null_mut();
        // SAFETY: the codec is live, the slices are valid for their lengths, and both out-pointers are valid.
        let status = unsafe {
            ffi::nj2k_shim_codec_dimensions_for(
                self.0.as_ptr(),
                data.as_ptr(),
                data.len(),
                indices.as_ptr(),
                count,
                &raw mut numcomps,
                &raw mut raw,
            )
        };
        if status != 0 {
            return Err(status);
        }
        let header = OwnedHeader(NonNull::new(raw).expect("the C++ shim did not return a header snapshot"));
        Ok(header.components(numcomps, true))
    }

    /// Read a header and also return ordered diagnostics from the read itself.
    pub fn read_header_with_diagnostics(&mut self, data: &[u8]) -> (Result<Header, i32>, Vec<HeaderDiagnostic>) {
        self.read_header_request(data, None, true)
    }

    fn read_header_request(
        &mut self,
        data: &[u8],
        reduce_after_header: Option<u32>,
        capture_diagnostics: bool,
    ) -> (Result<Header, i32>, Vec<HeaderDiagnostic>) {
        let mut info = ffi::HeaderInfo::default();
        let mut raw = std::ptr::null_mut();
        let reduce = reduce_after_header
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref);
        // SAFETY: the codec is live, data valid for its length, and reduce null or valid for the call.
        // The shim writes to valid out-pointers info and raw.
        let status = unsafe {
            ffi::nj2k_shim_codec_header(
                self.0.as_ptr(),
                data.as_ptr(),
                data.len(),
                reduce,
                u32::from(capture_diagnostics),
                &raw mut info,
                &raw mut raw,
            )
        };
        let header = NonNull::new(raw).map(OwnedHeader);
        let diagnostics = header.as_ref().map_or_else(Vec::new, OwnedHeader::diagnostics);
        if status != 0 {
            return (Err(status), diagnostics);
        }
        let header = header.expect("the C++ shim did not return a header snapshot");
        (
            Ok(Header {
                bounds: [info.x0, info.y0, info.x1, info.y1],
                tiles: [info.tx0, info.ty0, info.tdx, info.tdy, info.tw, info.th],
                layers: info.numlayers,
                min_resolutions: info.min_resolutions,
                progression: info.progression,
                color_space: info.color_space,
                components: header.components(info.numcomps, false),
                reduce_status: reduce_after_header.map(|_| info.reduce_status),
                dimensions: if info.dimensions_status == 0 {
                    Ok(header.components(info.numcomps, true))
                } else {
                    Err(info.dimensions_status)
                },
            }),
            diagnostics,
        )
    }

    /// The header from memory, then the decode into planar `int32` planes (each component `w x h`,
    /// one after the other): the status (`NJ2K_OK`, `NJ2K_TRUNCATED` or an error) and the samples.
    pub fn decode_i32(&mut self, data: &[u8]) -> (i32, Vec<i32>) {
        let mut prepared = self.prepare_i32(data);
        if prepared.status != 0 {
            return (prepared.status, Vec::new());
        }
        let need = prepared.samples_needed();
        let Ok(need_usize) = usize::try_from(need) else {
            return (-7, Vec::new());
        };
        let mut out = Vec::new();
        if out.try_reserve_exact(need_usize).is_err() {
            return (-7, out);
        }
        out.resize(need_usize, 0i32);
        let (st, count) = prepared.decode_into(&mut out);
        assert_eq!(count, need, "the header changed between two reads of the same bytes");
        (st, out)
    }

    /// Query the header once and keep its input and decoder settings for repeated decodes.
    pub fn prepare_i32<'a>(&'a mut self, data: &'a [u8]) -> PreparedI32<'a> {
        let (status, need) = self.decode_i32_into(data, None, None);
        PreparedI32 {
            codec: self,
            data,
            status,
            need,
        }
    }

    fn decode_i32_into(&mut self, data: &[u8], output: Option<&mut [i32]>, pool: Option<&BenchmarkPool>) -> (i32, u64) {
        let mut count = 0u64;
        let (ptr, cap) = output.map_or((std::ptr::null_mut(), 0), |out| (out.as_mut_ptr(), out.len() as u64));
        let pool = pool.map_or(std::ptr::null_mut(), BenchmarkPool::as_ptr);
        // SAFETY: the codec, slices and count are live, and the shim borrows a pool only for this call.
        // The shim only offsets the output pointer for nonempty planes after checking their length.
        let st = unsafe {
            ffi::nj2k_shim_codec_decode_i32(
                self.0.as_ptr(),
                data.as_ptr(),
                data.len(),
                ptr,
                cap,
                &raw mut count,
                pool,
            )
        };
        (st, count)
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

/// One input whose planar output size has been checked against the oracle header.
#[derive(Debug)]
pub struct PreparedI32<'a> {
    codec: &'a mut Codec,
    data: &'a [u8],
    status: i32,
    need: u64,
}

impl PreparedI32<'_> {
    /// The sample capacity required by the header, or zero if the header was refused.
    #[must_use]
    pub const fn samples_needed(&self) -> u64 {
        self.need
    }

    /// Decode into a reusable sample buffer. Returns the status and required sample count.
    pub fn decode_into(&mut self, output: &mut [i32]) -> (i32, u64) {
        self.decode_into_with_pool(output, None)
    }

    /// Decode into a reusable sample buffer, optionally lending the C++ benchmark pool for this call.
    pub fn decode_into_with_pool(&mut self, output: &mut [i32], pool: Option<&BenchmarkPool>) -> (i32, u64) {
        if self.status != 0 {
            return (self.status, self.need);
        }
        if (output.len() as u64) < self.need {
            return (-9, self.need);
        }
        self.codec.decode_i32_into(self.data, Some(output), pool)
    }
}

impl Drop for Codec {
    fn drop(&mut self) {
        // SAFETY: the codec came from nj2k_shim_codec_new and is freed once.
        unsafe { ffi::nj2k_shim_codec_free(self.0.as_ptr()) };
    }
}
