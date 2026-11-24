// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The measured sets: the six decoder dumps of the C++ repository (`tests/vectors/t1_bench`, what
//! `bench/nj2k_t1_dec.cpp` times) and the two synthetic 64 x 64 upload-texture block sets of
//! `bench/nj2k_enc_t1_ratio.cpp` ("lossy" and "lossless"), regenerated here the way that bench makes them.
//!
//! What a pass times is not the C++ benches': the decoder sets time `DecodeWorkspace::decode`, which stores the
//! coefficients into the tile buffer as well as decoding them (the C++ bench times `t1_decode_cblk` alone), and the
//! encoder sets time `EncodeWorkspace::encode`, which copies the samples in and the passes and bytes out as well as
//! coding the block (the C++ bench times `enc_t1_encode`, copies the samples outside the timed part and commits the
//! passes outside it). Compare builds of this benchmark with one another, not with those figures.

use std::hint::black_box;
use std::path::Path;

use neo_j2k::bench::{DecodeBlock, DecodeWorkspace, EncodeParams, EncodeWorkspace};
use neo_j2k::{
    DecodeParams, DecodeTarget, Decoder, DecoderConfig, EncodeParams as ImageParams, Encoder, ImageSamples, ImageView,
    OutputPlane, SampleType,
};
use neo_j2k_testkit::files::corpus_dir;
use neo_j2k_testkit::fnv::Fnv64;
use neo_j2k_testkit::mqng;

use crate::clock;

/// The 9/7 step the decoder dumps are stored with (the dumps do not say it; the work is the same for any value).
const STEP: f32 = 1.0;

/// One synthetic encoder block.
pub(crate) struct UploadBlock {
    samples: Vec<i32>,
    orientation: u8,
    level: usize,
    step_size: f32,
}

impl UploadBlock {
    fn params(&self, reversible: bool) -> EncodeParams {
        EncodeParams {
            orientation: self.orientation,
            level: self.level,
            component: 0,
            reversible,
            step_size: self.step_size,
            mct: false,
        }
    }
}

/// A set's blocks and what one pass over them does.
pub(crate) enum Set {
    Decode {
        name: String,
        reversible: bool,
        blocks: Vec<DecodeBlock>,
        workspace: DecodeWorkspace,
        out: Vec<i32>,
    },
    Encode {
        name: &'static str,
        reversible: bool,
        blocks: Vec<UploadBlock>,
        workspace: EncodeWorkspace,
    },
    /// One whole-image encode through the public API with a fresh encoder per pass, as the small-encoder rows of
    /// `small_image_gates` time it.
    Image(ImageSet),
    /// One whole small-image decode through the public API into caller-owned planes with a warmed decoder, as the
    /// small-decoder rows of `small_image_gates` time it.
    Small(Box<SmallDecode>),
}

/// A whole-image decode request: a small corpus codestream, a resolution reduction, whether its last two bytes (the
/// end-of-codestream marker) are cut, and the decoder and output planes that the request reuses from one pass to the next.
pub(crate) struct SmallDecode {
    name: String,
    data: Vec<u8>,
    decoder: Decoder,
    params: DecodeParams<'static>,
    bytes: Vec<u8>,
    planes: Vec<OutputPlane>,
}

impl SmallDecode {
    /// The request, with its decoder warmed and its output planes sized from one decode.
    fn new(file: &str, tag: &str, reduce: u32, cut: bool) -> Result<Self, String> {
        let path = corpus_dir().join(file);
        let mut data = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        if cut {
            data.truncate(data.len() - 2);
        }
        let mut decoder = Decoder::new(DecoderConfig {
            reduce,
            ..DecoderConfig::default()
        });
        let params = DecodeParams::default();
        let image = decoder
            .decode(&data, params)
            .map_err(|e| format!("{file}: {}", e.message().trim_end()))?;
        if image.truncated != cut {
            return Err(format!("{file}: truncated is {}, not {cut}", image.truncated));
        }
        let mut offset = 0;
        let mut planes = Vec::new();
        for (component, plane) in image.components.iter().enumerate() {
            let row_stride = plane.info().width as usize * size_of::<i32>();
            planes.push(OutputPlane {
                component: u32::try_from(component).map_err(|_| "too many components".to_owned())?,
                offset,
                row_stride: isize::try_from(row_stride).map_err(|_| "a plane too wide".to_owned())?,
                sample_stride: size_of::<i32>(),
            });
            offset += size_of_val(plane.samples());
        }
        let mut set = Self {
            name: format!("{}_{tag}", file.trim_end_matches(".j2k")),
            data,
            decoder,
            params,
            bytes: vec![0; offset],
            planes,
        };
        set.decode()?;
        Ok(set)
    }

    /// One decode into the request's planes; whether the codestream was cut.
    fn decode(&mut self) -> Result<bool, String> {
        self.decoder
            .decode_into(
                black_box(&self.data),
                self.params,
                SampleType::I32,
                &mut [DecodeTarget {
                    bytes: &mut self.bytes,
                    planes: &self.planes,
                }],
            )
            .map_err(|e| format!("small {}: {}", self.name, e.message().trim_end()))
    }
}

/// A whole-image encode request: the pixels of a square interleaved RGB image and the settings.
pub(crate) struct ImageSet {
    /// The set's prefix in the output: `image` (every pass kept) or `rated` (a rate target or a size cap).
    group: &'static str,
    name: String,
    side: u32,
    pixels: Vec<u8>,
    params: ImageParams<'static>,
}

impl ImageSet {
    /// The encoded codestream of one pass.
    fn encode(&self) -> Result<Vec<u8>, String> {
        let stride = self.side as usize * 3;
        let view = ImageView {
            width: self.side,
            height: self.side,
            flip_vertical: false,
            samples: ImageSamples::Interleaved {
                samples: &self.pixels,
                components: 3,
                row_stride: stride,
            },
        };
        Encoder::new()
            .encode(black_box(&view), &self.params)
            .map(<[u8]>::to_vec)
            .map_err(|e| format!("image {}: {}", self.name, e.message().trim_end()))
    }
}

impl Set {
    /// The set's name in the output (`decode/1024_lossy`, `encode/lossless`).
    pub(crate) fn name(&self) -> String {
        match self {
            Self::Decode { name, .. } => format!("decode/{name}"),
            Self::Encode { name, .. } => format!("encode/{name}"),
            Self::Image(set) => format!("{}/{}", set.group, set.name),
            Self::Small(set) => format!("small/{}", set.name),
        }
    }

    pub(crate) fn blocks(&self) -> usize {
        match self {
            Self::Decode { blocks, .. } => blocks.len(),
            Self::Encode { blocks, .. } => blocks.len(),
            Self::Image(_) | Self::Small(_) => 1,
        }
    }

    /// One pass over every block, as timed.
    pub(crate) fn pass(&mut self) {
        match self {
            Self::Decode {
                reversible,
                blocks,
                workspace,
                out,
                ..
            } => {
                for block in blocks.iter() {
                    black_box(workspace.decode(block, *reversible, STEP, out));
                    black_box(&*out);
                }
            }
            Self::Encode {
                reversible,
                blocks,
                workspace,
                ..
            } => {
                for block in blocks.iter() {
                    black_box(workspace.encode(&block.samples, 64, 64, &block.params(*reversible)));
                    black_box(workspace.block());
                }
            }
            Self::Small(set) => {
                black_box(set.decode().ok());
                black_box(set.bytes.as_slice());
            }
            Self::Image(set) => {
                let view = ImageView {
                    width: set.side,
                    height: set.side,
                    flip_vertical: false,
                    samples: ImageSamples::Interleaved {
                        samples: &set.pixels,
                        components: 3,
                        row_stride: set.side as usize * 3,
                    },
                };
                black_box(
                    Encoder::new()
                        .encode(black_box(&view), &set.params)
                        .map(<[u8]>::len)
                        .ok(),
                );
            }
        }
    }

    /// A hash of what one pass produces: the coefficients of every decoded block, or the passes and bytes of every
    /// encoded one (the C++ bench's digest). Equal hashes of two builds say they computed the same thing.
    ///
    /// # Errors
    /// A block that Tier-1 refuses: the sets are made of blocks it accepts.
    pub(crate) fn digest(&mut self) -> Result<u64, String> {
        let mut hash = Fnv64::new();
        match self {
            Self::Decode {
                reversible,
                blocks,
                workspace,
                out,
                ..
            } => {
                for (i, block) in blocks.iter().enumerate() {
                    if !workspace.decode(block, *reversible, STEP, out) {
                        return Err(format!("decoder block {i} is refused"));
                    }
                    for &v in &out[..block.samples()] {
                        hash.bytes(&v.to_le_bytes());
                    }
                }
            }
            Self::Encode {
                reversible,
                blocks,
                workspace,
                ..
            } => {
                for (i, block) in blocks.iter().enumerate() {
                    if !workspace.encode(&block.samples, 64, 64, &block.params(*reversible)) {
                        return Err(format!("encoder block {i} is refused"));
                    }
                    let encoded = workspace.block();
                    hash.bytes(&encoded.numbps.to_le_bytes());
                    let passes: Vec<_> = encoded.passes().collect();
                    hash.u32(u32::try_from(passes.len()).unwrap_or(u32::MAX));
                    for p in &passes {
                        hash.bytes(&p.rate.to_le_bytes());
                        hash.bytes(&p.len.to_le_bytes());
                        hash.bytes(&[u8::from(p.terminated)]);
                        hash.bytes(&p.distortion_bits.to_le_bytes());
                    }
                    hash.bytes(encoded.data);
                }
            }
            Self::Image(set) => hash.bytes(&set.encode()?),
            Self::Small(set) => {
                hash.bytes(&[u8::from(set.decode()?)]);
                hash.bytes(&set.bytes);
            }
        }
        Ok(hash.value())
    }
}

/// The decoder dumps of a folder (the C++ checkout's `tests/vectors/t1_bench`), by file name.
///
/// # Errors
/// What the dump reader reports.
pub(crate) fn decode_sets(dir: &Path) -> Result<Vec<Set>, String> {
    let dumps = mqng::dumps_in(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    if dumps.is_empty() {
        return Err(format!("{}: no .blk dumps", dir.display()));
    }
    Ok(dumps
        .into_iter()
        .map(|(file, blocks)| {
            let name = file.strip_suffix(".blk").unwrap_or(&file).to_owned();
            Set::Decode {
                reversible: name.contains("lossless"),
                name,
                blocks: blocks
                    .into_iter()
                    .map(|b| {
                        DecodeBlock::new(
                            (b.width, b.height),
                            (b.numbps, b.orient, b.roishift, b.cblksty),
                            &b.segments,
                            b.bytes,
                        )
                    })
                    .collect(),
                workspace: DecodeWorkspace::new(),
                out: vec![0; 4096],
            }
        })
        .collect())
}

/// The generator of `nj2k_enc_t1_ratio.cpp`.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        // the C++ keeps the low 32 bits of the shifted word
        u32::try_from(((z ^ (z >> 31)) >> 16) & 0xffff_ffff).unwrap_or(0)
    }

    fn unit(&mut self) -> f64 {
        (f64::from(self.next()) + 0.5) / 4_294_967_296.0
    }
}

/// `make_blocks` of `nj2k_enc_t1_ratio.cpp`: 64 x 64 blocks of a Laplacian-like high band, busy, medium and
/// quiet ones, an edge-like ridge inside each. `lossless` takes the 5/3's larger coefficients, else the 9/7's
/// small sparse ones with a dead zone.
pub(crate) fn upload_blocks(lossless: bool, count: u32) -> Vec<UploadBlock> {
    let mut rng = Rng(if lossless { 7001 } else { 3001 });
    (0..count)
        .map(|b| {
            let activity = 0.35 + 2.2 * rng.unit().powf(2.0);
            let scale = (if lossless { 24.0 } else { 5.0 }) * activity;
            let dead = if lossless { 0.0 } else { 0.6 };
            let mut samples = Vec::with_capacity(64 * 64);
            for y in 0..64u32 {
                for x in 0..64u32 {
                    let ridge =
                        1.0 + 3.0 * (-(f64::from(x) - 32.0 - 9.0 * (f64::from(y) * 0.12).sin()).abs() * 0.25).exp();
                    let mut v = -rng.unit().ln() * scale * ridge;
                    if v < dead {
                        v = 0.0;
                    }
                    // six fractional bits, as the encoder's samples have
                    #[allow(clippy::cast_possible_truncation, reason = "v * 64 is far below i32's range")]
                    let mut q = (v * 64.0) as i32;
                    if rng.next() & 1 == 1 {
                        q = -q;
                    }
                    samples.push(q);
                }
            }
            UploadBlock {
                samples,
                orientation: u8::try_from(b & 3).unwrap_or(0),
                level: ((b >> 2) % 5) as usize,
                step_size: 0.0625 + (b % 7) as f32 * 0.05,
            }
        })
        .collect()
}

/// The two encoder sets.
pub(crate) fn encode_sets(blocks: u32) -> Vec<Set> {
    [("lossy", false), ("lossless", true)]
        .into_iter()
        .map(|(name, reversible)| Set::Encode {
            name,
            reversible,
            blocks: upload_blocks(reversible, blocks),
            workspace: EncodeWorkspace::new(),
        })
        .collect()
}

/// The whole-image sets. The six `image/` sets are the small-encoder rows of `neo-j2k/tests/small_image_gates.rs`
/// (RGB, three sizes, lossless 5/3 and lossy 9/7, six resolutions, colour transform, every pass kept); the `rated/`
/// sets are requests with a rate target or a size cap, which keep the rate allocator's search in the code.
pub(crate) fn image_sets() -> Vec<Set> {
    let make =
        |group: &'static str, side: u32, reversible: bool, tweak: &dyn Fn(&mut ImageParams<'static>), tag: &str| {
            let stride = side as usize * 3;
            let pixels = (0..side as usize * stride)
                .map(|index| {
                    let x = (index % stride) / 3;
                    let y = index / stride;
                    let component = index % 3;
                    (3 * x + 5 * y + 13 * (x ^ y) + 23 * component).to_le_bytes()[0]
                })
                .collect();
            let mut params = ImageParams::default();
            params.reversible = reversible;
            params.mct = true;
            params.resolutions = Some(6);
            tweak(&mut params);
            Set::Image(ImageSet {
                group,
                name: format!("rgb{side}_{}{tag}", if reversible { "lossless" } else { "lossy" }),
                side,
                pixels,
                params,
            })
        };
    let mut sets = Vec::new();
    for side in [32, 64, 128] {
        for reversible in [true, false] {
            sets.push(make("image", side, reversible, &|_| {}, ""));
        }
    }
    sets.push(make(
        "rated",
        128,
        true,
        &|p| p.layer_ratios = vec![3.0, 1.5],
        "_layers",
    ));
    sets.push(make(
        "rated",
        128,
        false,
        &|p| p.layer_ratios = vec![30.0, 10.0, 0.0],
        "_layers",
    ));
    sets.push(make("rated", 128, false, &|p| p.max_codestream_size = 6000, "_cap"));
    sets
}

/// The whole small-decode sets: the nine rows of the small-decoder suite of `neo-j2k/tests/small_image_gates.rs` (three
/// corpus images, each whole, at resolution reduction 2, and with the end-of-codestream marker cut). Reduced decodes of
/// these images are mostly blocks of at most four rows, which the code-block sets do not show.
///
/// # Errors
/// A corpus file that cannot be read or decoded.
pub(crate) fn small_decode_sets() -> Result<Vec<Set>, String> {
    let mut sets = Vec::new();
    for file in ["sl_32_rgba_53.j2k", "sl_64_rgb_97.j2k", "sl_128_rgba_97.j2k"] {
        for (tag, reduce, cut) in [("whole", 0, false), ("reduce2", 2, false), ("missing_eoc", 0, true)] {
            sets.push(Set::Small(Box::new(SmallDecode::new(file, tag, reduce, cut)?)));
        }
    }
    Ok(sets)
}

/// What one set measured.
pub(crate) struct Measurement {
    /// The clock units of one pass over the set's blocks (the median of the timed units).
    pub(crate) per_pass: u64,
    /// The passes in a timed unit.
    pub(crate) inner: u64,
}

/// Times the set: a warm-up pass, a calibration pass that sets the passes in a unit so that a unit takes at least
/// `min_units` of the clock (the C++ bench's `--inner auto`), then `units` timed units; the median per pass.
pub(crate) fn measure(set: &mut Set, min_units: u64, units: usize) -> Measurement {
    set.pass();
    let t0 = clock::start();
    set.pass();
    let one = clock::stop().saturating_sub(t0).max(1);
    let inner = min_units.div_ceil(one).max(1);
    let mut per_pass: Vec<u64> = (0..units.max(1))
        .map(|_| {
            let t = clock::start();
            for _ in 0..inner {
                set.pass();
            }
            clock::stop().saturating_sub(t) / inner
        })
        .collect();
    per_pass.sort_unstable();
    Measurement {
        per_pass: per_pass[per_pass.len() / 2],
        inner,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_upload_blocks_are_the_same_every_time() {
        for lossless in [false, true] {
            let a = upload_blocks(lossless, 8);
            let b = upload_blocks(lossless, 8);
            assert_eq!(a.len(), 8);
            assert!(
                a.iter()
                    .zip(&b)
                    .all(|(x, y)| x.samples == y.samples && x.samples.len() == 4096)
            );
            assert!(a.iter().any(|x| x.samples.iter().any(|&s| s != 0)));
        }
        let mut one = encode_sets(16);
        let mut two = encode_sets(16);
        for (a, b) in one.iter_mut().zip(&mut two) {
            let digest = a.digest().expect("the upload blocks encode");
            assert_eq!(Ok(digest), b.digest());
            assert_eq!(a.blocks(), 16);
        }
    }

    #[test]
    fn the_image_sets_encode_and_are_the_same_every_time() {
        let mut one = image_sets();
        let mut two = image_sets();
        assert_eq!(one.len(), 9);
        for (a, b) in one.iter_mut().zip(&mut two) {
            let digest = a.digest().expect("the image requests encode");
            assert_eq!(Ok(digest), b.digest(), "{}", a.name());
            a.pass();
        }
        assert_eq!(one[0].name(), "image/rgb32_lossless");
        assert_eq!(one[8].name(), "rated/rgb128_lossy_cap");
    }

    #[test]
    fn the_small_decode_sets_decode_and_are_the_same_every_time() {
        let mut one = small_decode_sets().expect("the corpus images decode");
        let mut two = small_decode_sets().expect("the corpus images decode");
        assert_eq!(one.len(), 9);
        for (a, b) in one.iter_mut().zip(&mut two) {
            let digest = a.digest().expect("the small decode requests decode");
            assert_eq!(Ok(digest), b.digest(), "{}", a.name());
            a.pass();
        }
        assert_eq!(one[0].name(), "small/sl_32_rgba_53_whole");
        assert_eq!(one[1].name(), "small/sl_32_rgba_53_reduce2");
        assert_eq!(one[8].name(), "small/sl_128_rgba_97_missing_eoc");
        // the reduction and the cut change what is decoded
        let digests: Vec<_> = one.iter_mut().take(3).map(|s| s.digest().unwrap()).collect();
        assert!(digests[0] != digests[1] && digests[0] != digests[2] && digests[1] != digests[2]);
    }

    #[test]
    fn a_pass_leaves_the_digest_alone_and_a_measurement_is_positive() {
        let mut sets = encode_sets(8);
        let before = sets[0].digest().unwrap();
        let m = measure(&mut sets[0], 1_000, 3);
        assert!(m.per_pass > 0 && m.inner >= 1);
        assert_eq!(sets[0].digest().unwrap(), before);
        assert_eq!(sets[0].name(), "encode/lossy");
    }
}
