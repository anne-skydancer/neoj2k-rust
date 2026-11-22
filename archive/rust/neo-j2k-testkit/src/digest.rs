// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! `nj2k_diff`'s identities: an outcome's hash over its geometry and samples (`replay.inl`
//! `finish`), the viewer's 8-bit image and its hash (`replay_viewer`), the value a reference
//! table's `whole` line holds, and the digest of a `cuts` line (`nj2k_diff.cpp` `cut_digest`).

use crate::fnv::Fnv64;
use crate::request::{Backend, Mode, Outcome, Request};

/// The identity hash of a decoded image: its area, components and colour space, each component's
/// geometry and then its samples (as `u32`); 0 when the request failed.
#[must_use]
pub fn outcome_hash(o: &Outcome) -> u64 {
    if !o.ok {
        return 0;
    }
    let mut h = Fnv64::new();
    for v in [o.x0, o.y0, o.x1, o.y1, o.numcomps, o.color_space.cast_unsigned()] {
        h.u32(v);
    }
    for (c, k) in o.comps.iter().enumerate() {
        for v in [
            k.w,
            k.h,
            k.dx,
            k.dy,
            k.x0,
            k.y0,
            k.prec,
            k.sgnd,
            k.factor,
            k.resno_decoded,
        ] {
            h.u32(v);
        }
        if let Some(plane) = o.planes.get(c) {
            for &s in plane {
                h.u32(s.cast_unsigned());
            }
        }
    }
    h.value()
}

/// The viewer's raw image (`LLImageRaw`): up to four components interleaved, the bottom row first,
/// the low byte of each sample. `None` when the outcome failed or has no component.
#[must_use]
pub fn viewer_u8_image(o: &Outcome) -> Option<Vec<u8>> {
    if !o.ok || o.comps.is_empty() {
        return None;
    }
    let channels = o.comps.len().min(4);
    let (w, h) = (o.comps[0].w as usize, o.comps[0].h as usize);
    let mut raw = vec![0u8; w * h * channels];
    for (c, plane) in o.planes.iter().take(channels).enumerate() {
        let mut at = c;
        for y in (0..h).rev() {
            for &s in &plane[y * w..(y + 1) * w] {
                // the low byte, as the viewer's static_cast<std::uint8_t>
                raw[at] = s.to_le_bytes()[0];
                at += channels;
            }
        }
    }
    Some(raw)
}

/// FNV-1a of the viewer's raw image; 0 when there is none.
#[must_use]
pub fn viewer_u8_hash(o: &Outcome) -> u64 {
    viewer_u8_image(o).map_or(0, |raw| {
        let mut h = Fnv64::new();
        h.bytes(&raw);
        h.value()
    })
}

/// The hash a reference table's `whole` line holds: the outcome's hash, combined with the 8-bit
/// image's in viewer mode; 0 when the request failed.
#[must_use]
pub fn whole_value(o: &Outcome, mode: Mode) -> u64 {
    if !o.ok {
        0
    } else if mode == Mode::Viewer {
        outcome_hash(o) ^ viewer_u8_hash(o)
    } else {
        outcome_hash(o)
    }
}

/// A `cuts` line's numbers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CutDigest {
    /// The digest over every cut's size, result and value.
    pub digest: u64,
    /// Cuts decoded.
    pub count: usize,
    /// Cuts that decoded.
    pub decoded: usize,
}

/// Every prefix of `data` from `from` bytes in steps of `step`, short of the whole file, decoded at
/// `reduce` in `mode`. `step` must not be 0.
pub fn cut_digest(
    backend: &mut dyn Backend,
    data: &[u8],
    mode: Mode,
    reduce: i32,
    from: usize,
    step: usize,
) -> CutDigest {
    assert!(step > 0, "a cut step of 0");
    let mut h = Fnv64::new();
    let mut d = CutDigest::default();
    for n in (from..data.len()).step_by(step) {
        let o = backend.decode(&Request::new(&data[..n], reduce, mode));
        h.u64(n as u64);
        h.u32(u32::from(o.ok));
        h.u64(whole_value(&o, mode));
        d.count += 1;
        d.decoded += usize::from(o.ok);
    }
    d.digest = h.value();
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::Component;

    fn image() -> Outcome {
        let comp = Component {
            w: 2,
            h: 2,
            dx: 1,
            dy: 1,
            prec: 8,
            ..Component::default()
        };
        Outcome {
            ok: true,
            x1: 2,
            y1: 2,
            numcomps: 2,
            comps: vec![comp, comp],
            planes: vec![vec![1, 2, 3, 4], vec![256 + 5, -1, 7, 8]],
            ..Outcome::default()
        }
    }

    #[test]
    fn viewer_image_is_bottom_up_interleaved_low_bytes() {
        assert_eq!(viewer_u8_image(&image()), Some(vec![3, 7, 4, 8, 1, 5, 2, 255]));
    }

    #[test]
    fn failed_outcomes_hash_to_zero() {
        let o = Outcome { ok: false, ..image() };
        assert_eq!(outcome_hash(&o), 0);
        assert_eq!(whole_value(&o, Mode::Viewer), 0);
        assert_eq!(viewer_u8_hash(&o), 0);
    }

    #[test]
    fn viewer_value_combines_both_hashes() {
        let o = image();
        assert_eq!(whole_value(&o, Mode::Plain), outcome_hash(&o));
        assert_eq!(whole_value(&o, Mode::Viewer), outcome_hash(&o) ^ viewer_u8_hash(&o));
    }

    struct Fixed;
    impl Backend for Fixed {
        fn name(&self) -> &str {
            "fixed"
        }
        fn decode(&mut self, rq: &Request<'_>) -> Outcome {
            Outcome {
                ok: rq.data.len().is_multiple_of(2),
                ..image()
            }
        }
    }

    #[test]
    fn cut_digest_counts_every_step() {
        let data = [0u8; 10];
        let d = cut_digest(&mut Fixed, &data, Mode::Plain, 0, 1, 3);
        // cuts 1, 4, 7: one decodes
        assert_eq!((d.count, d.decoded), (3, 1));
        let mut h = Fnv64::new();
        for n in [1u64, 4, 7] {
            h.u64(n);
            h.u32(u32::from(n % 2 == 0));
            h.u64(if n % 2 == 0 { outcome_hash(&image()) } else { 0 });
        }
        assert_eq!(d.digest, h.value());
    }
}
