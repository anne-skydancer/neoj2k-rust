// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The vertical 9/7 schedule follows v97_lift_rows in src/dwt.cpp. See LICENSE.OpenJPEG.

use super::{ALPHA, BETA, DELTA, GAMMA, K, TWO_INV_K, from_sample, lift_horizontal_97, to_sample};

enum Rows<'a, 'samples> {
    Flat(&'a mut [i32]),
    Stripe(&'a mut [&'samples mut [i32]]),
}

struct Band<'a, 'samples> {
    rows: Rows<'a, 'samples>,
    width: usize,
    stride: usize,
}

impl Band<'_, '_> {
    fn row(&self, index: usize) -> &[i32] {
        match &self.rows {
            Rows::Flat(data) => {
                let start = index * self.stride;
                &data[start..start + self.width]
            }
            Rows::Stripe(rows) => rows[index],
        }
    }

    fn row_mut(&mut self, index: usize) -> &mut [i32] {
        match &mut self.rows {
            Rows::Flat(data) => {
                let start = index * self.stride;
                &mut data[start..start + self.width]
            }
            Rows::Stripe(rows) => rows[index],
        }
    }

    fn rows<const N: usize>(&mut self, indices: [usize; N]) -> [&mut [i32]; N] {
        match &mut self.rows {
            Rows::Flat(data) => data
                .get_disjoint_mut(indices.map(|index| {
                    let start = index * self.stride;
                    start..start + self.width
                }))
                .expect("a sweep works on disjoint rows"),
            Rows::Stripe(rows) => rows
                .get_disjoint_mut(indices)
                .expect("a sweep works on disjoint rows")
                .map(|row| &mut **row),
        }
    }
}

#[derive(Clone, Copy)]
struct Step {
    interior: usize,
    count: usize,
    offset: usize,
    coefficient: f32,
}

impl Step {
    fn new(count: usize, other: usize, offset: usize, coefficient: f32) -> Self {
        let interior = count.min(other - offset);
        Self {
            interior,
            count,
            offset,
            coefficient,
        }
    }

    fn ready(self, index: usize, targets: usize, sources: usize) -> bool {
        index < self.count
            && index < targets
            && if index < self.interior {
                index + self.offset < sources
            } else {
                self.interior + self.offset <= sources
            }
    }

    fn apply(self, band: &mut Band<'_, '_>, other: &Band<'_, '_>, index: usize) {
        let row = band.row_mut(index);
        let c = self.coefficient;
        if index < self.interior {
            let right = index + self.offset;
            let left = other.row(right.saturating_sub(1));
            let right = other.row(right);
            for ((out, &a), &b) in row.iter_mut().zip(left).zip(right) {
                *out = to_sample(lift(a, b, from_sample(*out), c));
            }
        } else {
            let adjacent = other.row(self.interior + self.offset - 1);
            for (out, &sample) in row.iter_mut().zip(adjacent) {
                *out = to_sample(from_sample(*out) + (c + c) * from_sample(sample));
            }
        }
    }
}

#[inline]
fn lift(a: i32, b: i32, sample: f32, coefficient: f32) -> f32 {
    sample + (from_sample(a) + from_sample(b)) * coefficient
}

fn scale(row: &mut [i32], coefficient: f32) {
    for sample in row {
        *sample = to_sample(from_sample(*sample) * coefficient);
    }
}

fn sweep_even(low: [&mut [i32]; 3], high: [&mut [i32]; 3]) {
    let [l0, l1, l2] = low;
    let [h0, h1, h2] = high;
    #[cfg(not(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2")))]
    let begin = 0;
    #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
    let begin = crate::simd::dwt::sweep_97_rows::<false>(
        &mut [&mut *l0, &mut *l1, &mut *l2, &mut []],
        &mut [&mut *h0, &mut *h1, &mut *h2],
    );
    for x in begin..l0.len() {
        let hk = to_sample(from_sample(h0[x]) * TWO_INV_K);
        let d = to_sample(lift(h1[x], hk, from_sample(l0[x]) * K, -DELTA));
        let g = to_sample(lift(l1[x], d, from_sample(h1[x]), -GAMMA));
        let b = to_sample(lift(h2[x], g, from_sample(l1[x]), -BETA));
        let a = to_sample(lift(l2[x], b, from_sample(h2[x]), -ALPHA));
        l0[x] = d;
        h0[x] = hk;
        h1[x] = g;
        l1[x] = b;
        h2[x] = a;
    }
}

fn sweep_odd(low: [&mut [i32]; 4], high: [&mut [i32]; 3]) {
    let [l0, l1, l2, l3] = low;
    let [h0, h1, h2] = high;
    #[cfg(not(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2")))]
    let begin = 0;
    #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
    let begin = crate::simd::dwt::sweep_97_rows::<true>(
        &mut [&mut *l0, &mut *l1, &mut *l2, &mut *l3],
        &mut [&mut *h0, &mut *h1, &mut *h2],
    );
    for x in begin..l0.len() {
        let hk = to_sample(from_sample(h0[x]) * TWO_INV_K);
        let d = to_sample(lift(h1[x], hk, from_sample(l1[x]), -DELTA));
        let g = to_sample(lift(l2[x], d, from_sample(h1[x]), -GAMMA));
        let b = to_sample(lift(h2[x], g, from_sample(l2[x]), -BETA));
        let a = to_sample(lift(l3[x], b, from_sample(h2[x]), -ALPHA));
        l0[x] = to_sample(from_sample(l0[x]) * K);
        h0[x] = hk;
        l1[x] = d;
        h1[x] = g;
        l2[x] = b;
        h2[x] = a;
    }
}

/// Scale and lift whole rows in place, keeping each band in its original half of the buffer.
pub(super) fn lift_columns<const FUSE_HORIZONTAL: bool>(
    data: &mut [i32],
    width: usize,
    height: usize,
    stride: usize,
    parity: usize,
    x_parity: usize,
) {
    let low_len = height / 2 + height % 2 * (1 - parity);
    let high_len = height - low_len;
    let (low, high) = data.split_at_mut(low_len * stride);
    let low = Band {
        rows: Rows::Flat(low),
        width,
        stride,
    };
    let high = Band {
        rows: Rows::Flat(high),
        width,
        stride,
    };
    lift_bands::<FUSE_HORIZONTAL>(low, high, low_len, high_len, parity, x_parity);
}

pub(super) fn lift_stripe(rows: &mut [&mut [i32]], parity: usize) {
    let height = rows.len();
    let low_len = height / 2 + height % 2 * (1 - parity);
    let high_len = height - low_len;
    let (low, high) = rows.split_at_mut(low_len);
    let low = Band {
        rows: Rows::Stripe(low),
        width: 0,
        stride: 0,
    };
    let high = Band {
        rows: Rows::Stripe(high),
        width: 0,
        stride: 0,
    };
    lift_bands::<false>(low, high, low_len, high_len, parity, 0);
}

fn lift_bands<'a, 'samples, const FUSE_HORIZONTAL: bool>(
    mut low: Band<'a, 'samples>,
    mut high: Band<'a, 'samples>,
    low_len: usize,
    high_len: usize,
    parity: usize,
    x_parity: usize,
) {
    let steps = [
        Step::new(low_len, high_len, parity, -DELTA),
        Step::new(high_len, low_len, 1 - parity, -GAMMA),
        Step::new(low_len, high_len, parity, -BETA),
        Step::new(high_len, low_len, 1 - parity, -ALPHA),
    ];
    let mut next = [0; 4];
    for k in 0..low_len.max(high_len) {
        if FUSE_HORIZONTAL {
            // A vertical lift only reads rows that have entered the pipeline.
            if k < low_len {
                lift_horizontal_97(low.row_mut(k), x_parity);
            }
            if k < high_len {
                lift_horizontal_97(high.row_mut(k), x_parity);
            }
        }
        if k < low_len.min(high_len)
            && parity == 0
            && k >= 2
            && next == [k, k - 1, k - 1, k - 2]
            && k < steps[0].interior
            && k - 1 < steps[1].interior
            && k - 2 < steps[3].interior
        {
            sweep_even(low.rows([k, k - 1, k - 2]), high.rows([k, k - 1, k - 2]));
            next = [k + 1, k, k, k - 1];
        } else if k < low_len.min(high_len)
            && parity == 1
            && k >= 3
            && next == [k - 1, k - 1, k - 2, k - 2]
            && k - 1 < steps[0].interior
            && k - 1 < steps[1].interior
            && k - 2 < steps[3].interior
        {
            sweep_odd(low.rows([k, k - 1, k - 2, k - 3]), high.rows([k, k - 1, k - 2]));
            next = [k, k, k - 1, k - 1];
        } else {
            if k < low_len {
                scale(low.row_mut(k), K);
            }
            if k < high_len {
                scale(high.row_mut(k), TWO_INV_K);
            }
        }

        // At the edges there may be more or fewer than one target ready for a lift.
        for (index, step) in steps.iter().enumerate() {
            let targets = match index {
                0 => (k + 1).min(low_len),
                1 => (k + 1).min(high_len),
                _ => next[index - 2],
            };
            let sources = if index == 0 {
                (k + 1).min(high_len)
            } else {
                next[index - 1]
            };
            while step.ready(next[index], targets, sources) {
                let (band, other) = if index % 2 == 0 {
                    (&mut low, &high)
                } else {
                    (&mut high, &low)
                };
                step.apply(band, other, next[index]);
                next[index] += 1;
            }
        }
    }
    debug_assert_eq!(next, [low_len, high_len, low_len, high_len]);
}
