// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Eight independent inverse wavelet lines expressed as safe lane arrays.
//! The lifting order is the scalar DWT's.

use super::lanes::*;

use crate::dwt::{ALPHA, BETA, DELTA, GAMMA, K, TWO_INV_K};

pub(crate) const LANES: usize = 8;
type Lane = [i32; LANES];

/// Transform eight rows or columns with one sample per lane. The source lines hold low samples
/// followed by high samples. `sample_stride` steps through a line, `lane_stride` steps across
/// lanes, and `scratch` has a lane group for every sample of one line.
pub(crate) fn inverse(
    data: &mut [i32],
    sample_stride: usize,
    lane_stride: usize,
    parity: usize,
    reversible: bool,
    scratch: &mut [Lane],
) {
    let len = scratch.len();
    if len < 2 {
        if len == 1 && reversible && parity == 1 {
            for lane in 0..LANES {
                data[lane * lane_stride] /= 2;
            }
        }
        return;
    }

    let low_len = len / 2 + (len % 2) * (1 - parity);
    for (index, lanes) in scratch.iter_mut().enumerate() {
        let input = index / 2 + usize::from(index % 2 != parity) * low_len;
        let start = input * sample_stride;
        if lane_stride == 1 {
            lanes.copy_from_slice(&data[start..start + LANES]);
        } else {
            for (lane, sample) in lanes.iter_mut().enumerate() {
                *sample = data[start + lane * lane_stride];
            }
        }
    }

    if reversible {
        inverse_53(scratch, parity);
    } else {
        inverse_97(scratch, parity);
    }

    for (index, lanes) in scratch.iter().enumerate() {
        let start = index * sample_stride;
        if lane_stride == 1 {
            data[start..start + LANES].copy_from_slice(lanes);
        } else {
            for (lane, &sample) in lanes.iter().enumerate() {
                data[start + lane * lane_stride] = sample;
            }
        }
    }
}

/// Transform eight contiguous 9/7 rows using dense low/high bands in the existing scratch.
pub(crate) fn inverse_97_rows(data: &mut [i32], stride: usize, parity: usize, scratch: &mut [Lane]) {
    inverse_97_rows_dense(data, stride, parity, scratch);
}

/// Transform eight columns whose rows are borrowed separately by an executor job.
pub(crate) fn inverse_stripe(
    rows: &mut [&mut [i32]],
    column: usize,
    parity: usize,
    reversible: bool,
    scratch: &mut [Lane],
) {
    let len = scratch.len();
    let low_len = len / 2 + (len % 2) * (1 - parity);
    let columns = column..column + LANES;
    for (index, lanes) in scratch.iter_mut().enumerate() {
        let row = index / 2 + usize::from(index % 2 != parity) * low_len;
        lanes.copy_from_slice(&rows[row][columns.clone()]);
    }
    if reversible {
        inverse_53(scratch, parity);
    } else {
        inverse_97(scratch, parity);
    }
    for (row, lanes) in rows.iter_mut().zip(scratch) {
        row[columns.clone()].copy_from_slice(lanes);
    }
}

fn read(lanes: &Lane) -> IntLanes {
    *lanes
}

fn write(lanes: &mut Lane, value: IntLanes) {
    *lanes = value;
}

fn inverse_53(line: &mut [Lane], parity: usize) {
    let last = line.len() - 1;
    // Complete 16-column groups double the edge neighbour with wrapping arithmetic. The two-
    // sample odd-origin case is the scalar shortcut in the reference for both groups and tails.
    let full_group_edge = parity == 0 || last > 1;
    let one = splat_i32(1);
    let two = splat_i32(2);
    for index in (parity..=last).step_by(2) {
        let previous = read(&line[index.abs_diff(1)]);
        let next = read(&line[if index == last { last - 1 } else { index + 1 }]);
        let adjustment = if (index == 0 || index == last) && !full_group_edge {
            arithmetic_right::<1>(add_i32(previous, one))
        } else {
            arithmetic_right::<2>(add_i32(add_i32(previous, next), two))
        };
        let sample = sub_i32(read(&line[index]), adjustment);
        write(&mut line[index], sample);
    }
    for index in ((1 - parity)..=last).step_by(2) {
        let previous = read(&line[index.abs_diff(1)]);
        let adjustment = if index == 0 || index == last {
            previous
        } else {
            arithmetic_right::<1>(add_i32(previous, read(&line[index + 1])))
        };
        let sample = add_i32(read(&line[index]), adjustment);
        write(&mut line[index], sample);
    }
}

fn read_float(lanes: &Lane) -> FloatLanes {
    from_float_bits(read(lanes))
}

fn write_float(lanes: &mut Lane, value: FloatLanes) {
    write(lanes, float_bits(value));
}

pub(crate) fn sweep_97_rows<const ODD: bool>(low: &mut [&mut [i32]; 4], high: &mut [&mut [i32]; 3]) -> usize {
    sweep_97_avx2::<ODD>(low, high)
}

fn read_row(row: &[i32], column: usize) -> FloatLanes {
    read_float(row[column..column + LANES].try_into().expect("an AVX2 group"))
}

fn write_row(row: &mut [i32], column: usize, samples: FloatLanes) {
    write_float(
        (&mut row[column..column + LANES]).try_into().expect("an AVX2 group"),
        samples,
    );
}

#[inline(always)]
fn lift_rows(sample: FloatLanes, a: FloatLanes, b: FloatLanes, coefficient: FloatLanes) -> FloatLanes {
    add_f32(sample, mul_f32(add_f32(a, b), coefficient))
}

fn sweep_97_avx2<const ODD: bool>(low: &mut [&mut [i32]; 4], high: &mut [&mut [i32]; 3]) -> usize {
    let [l0, l1, l2, l3] = low;
    let [h0, h1, h2] = high;
    let width = l0.len();
    let end = width / LANES * LANES;
    let (k, two_inv_k) = (splat_f32(K), splat_f32(TWO_INV_K));
    let (delta, gamma) = (splat_f32(-DELTA), splat_f32(-GAMMA));
    let (beta, alpha) = (splat_f32(-BETA), splat_f32(-ALPHA));
    for x in (0..end).step_by(LANES) {
        let lk = mul_f32(read_row(l0, x), k);
        let hk = mul_f32(read_row(h0, x), two_inv_k);
        let hk1 = read_row(h1, x);
        let hk2 = read_row(h2, x);
        let lk1 = read_row(l1, x);
        let lk2 = read_row(l2, x);
        if ODD {
            let d = lift_rows(lk1, hk1, hk, delta);
            let g = lift_rows(hk1, lk2, d, gamma);
            let b = lift_rows(lk2, hk2, g, beta);
            let a = lift_rows(hk2, read_row(l3, x), b, alpha);
            write_row(l0, x, lk);
            write_row(l1, x, d);
            write_row(l2, x, b);
            write_row(h1, x, g);
            write_row(h2, x, a);
        } else {
            let d = lift_rows(lk, hk1, hk, delta);
            let g = lift_rows(hk1, lk1, d, gamma);
            let b = lift_rows(lk1, hk2, g, beta);
            let a = lift_rows(hk2, lk2, b, alpha);
            write_row(l0, x, d);
            write_row(l1, x, b);
            write_row(h1, x, g);
            write_row(h2, x, a);
        }
        write_row(h0, x, hk);
    }
    end
}

#[inline(always)]
fn lift_97(line: &mut [Lane], parity: usize, coefficient: f32) {
    let last = line.len() - 1;
    let scale = splat_f32(coefficient);
    for index in (parity..last).step_by(2) {
        let sum = add_f32(read_float(&line[index.abs_diff(1)]), read_float(&line[index + 1]));
        let delta = mul_f32(sum, scale);
        let sample = add_f32(read_float(&line[index]), delta);
        write_float(&mut line[index], sample);
    }
    if last % 2 == parity {
        let delta = mul_f32(add_f32(scale, scale), read_float(&line[last - 1]));
        let sample = add_f32(read_float(&line[last]), delta);
        write_float(&mut line[last], sample);
    }
}

fn inverse_97(line: &mut [Lane], parity: usize) {
    let low = splat_f32(K);
    let high = splat_f32(TWO_INV_K);
    for sample in line.iter_mut().skip(parity).step_by(2) {
        let scaled = mul_f32(read_float(sample), low);
        write_float(sample, scaled);
    }
    for sample in line.iter_mut().skip(1 - parity).step_by(2) {
        let scaled = mul_f32(read_float(sample), high);
        write_float(sample, scaled);
    }
    lift_97(line, parity, -DELTA);
    lift_97(line, 1 - parity, -GAMMA);
    lift_97(line, parity, -BETA);
    lift_97(line, 1 - parity, -ALPHA);
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn fill_dense(
    out: &mut [Lane],
    scale: f32,
    a: &[i32],
    b: &[i32],
    c: &[i32],
    d: &[i32],
    e: &[i32],
    f: &[i32],
    g: &[i32],
    h: &[i32],
) {
    for (target, (((((((&a, &b), &c), &d), &e), &f), &g), &h)) in out
        .iter_mut()
        .zip(a.iter().zip(b).zip(c).zip(d).zip(e).zip(f).zip(g).zip(h))
    {
        *target = [
            (f32::from_bits(a.cast_unsigned()) * scale).to_bits().cast_signed(),
            (f32::from_bits(b.cast_unsigned()) * scale).to_bits().cast_signed(),
            (f32::from_bits(c.cast_unsigned()) * scale).to_bits().cast_signed(),
            (f32::from_bits(d.cast_unsigned()) * scale).to_bits().cast_signed(),
            (f32::from_bits(e.cast_unsigned()) * scale).to_bits().cast_signed(),
            (f32::from_bits(f.cast_unsigned()) * scale).to_bits().cast_signed(),
            (f32::from_bits(g.cast_unsigned()) * scale).to_bits().cast_signed(),
            (f32::from_bits(h.cast_unsigned()) * scale).to_bits().cast_signed(),
        ];
    }
}

#[inline(always)]
fn lift_dense(mut band: &mut [Lane], neighbour: &[Lane], first: bool, coefficient: f32) {
    let scale = splat_f32(coefficient);
    if first {
        let (target, rest) = band.split_first_mut().unwrap();
        let n = from_float_bits(neighbour[0]);
        *target = float_bits(add_f32(from_float_bits(*target), mul_f32(add_f32(n, n), scale)));
        band = rest;
    }
    let span = band.len().min(neighbour.len() - 1);
    let (body, tail) = band.split_at_mut(span);
    for ((target, &a), &b) in body
        .as_flattened_mut().iter_mut()
        .zip(neighbour.as_flattened())
        .zip(neighbour[1..].as_flattened())
    {
        let sample = f32::from_bits(target.cast_unsigned());
        let a = f32::from_bits(a.cast_unsigned());
        let b = f32::from_bits(b.cast_unsigned());
        *target = (sample + (a + b) * coefficient).to_bits().cast_signed();
    }

    if let [last] = tail {
        let product = mul_f32(
            splat_f32(coefficient + coefficient),
            from_float_bits(neighbour[neighbour.len() - 1]),
        );
        *last = float_bits(add_f32(from_float_bits(*last), product));
    }
}

#[inline(always)]
// Separate mutable row arguments preserve disjoint destination streams for auto-vectorization.
#[allow(clippy::too_many_arguments)]
fn scatter_dense(
    first: &[Lane],
    second: &[Lane],
    a: &mut [i32],
    b: &mut [i32],
    c: &mut [i32],
    d: &mut [i32],
    e: &mut [i32],
    f: &mut [i32],
    g: &mut [i32],
    h: &mut [i32],
) {
    let odd = !a.len().is_multiple_of(2);
    for ((((((((a, b), c), d), e), f), g), h), (lo, hi)) in a
        .as_chunks_mut::<2>()
        .0
        .iter_mut()
        .zip(b.as_chunks_mut::<2>().0.iter_mut())
        .zip(c.as_chunks_mut::<2>().0.iter_mut())
        .zip(d.as_chunks_mut::<2>().0.iter_mut())
        .zip(e.as_chunks_mut::<2>().0.iter_mut())
        .zip(f.as_chunks_mut::<2>().0.iter_mut())
        .zip(g.as_chunks_mut::<2>().0.iter_mut())
        .zip(h.as_chunks_mut::<2>().0.iter_mut())
        .zip(first.iter().zip(second))
    {
        *a = [lo[0], hi[0]];
        *b = [lo[1], hi[1]];
        *c = [lo[2], hi[2]];
        *d = [lo[3], hi[3]];
        *e = [lo[4], hi[4]];
        *f = [lo[5], hi[5]];
        *g = [lo[6], hi[6]];
        *h = [lo[7], hi[7]];
    }
    if odd {
        let last = first[first.len() - 1];
        *a.last_mut().unwrap() = last[0];
        *b.last_mut().unwrap() = last[1];
        *c.last_mut().unwrap() = last[2];
        *d.last_mut().unwrap() = last[3];
        *e.last_mut().unwrap() = last[4];
        *f.last_mut().unwrap() = last[5];
        *g.last_mut().unwrap() = last[6];
        *h.last_mut().unwrap() = last[7];
    }
}

fn inverse_97_rows_dense(data: &mut [i32], stride: usize, parity: usize, scratch: &mut [Lane]) {
    // Dense bands keep every lane group contiguous without increasing the eight-row scratch.
    let len = scratch.len();
    let low_len = len / 2 + len % 2 * (1 - parity);
    let rows: [&[i32]; 8] = core::array::from_fn(|r| &data[r * stride..r * stride + len]);
    let (low, high) = scratch.split_at_mut(low_len);
    if parity==0 {
    fill_raw(
        low,
        &rows[0][..low_len],
        &rows[1][..low_len],
        &rows[2][..low_len],
        &rows[3][..low_len],
        &rows[4][..low_len],
        &rows[5][..low_len],
        &rows[6][..low_len],
        &rows[7][..low_len],
    );
    fill_raw(
        high,
        &rows[0][low_len..],
        &rows[1][low_len..],
        &rows[2][low_len..],
        &rows[3][low_len..],
        &rows[4][low_len..],
        &rows[5][low_len..],
        &rows[6][low_len..],
        &rows[7][low_len..],
    );

        fused_even(low,high);
    } else {
    fill_dense(
        low,
        K,
        &rows[0][..low_len],
        &rows[1][..low_len],
        &rows[2][..low_len],
        &rows[3][..low_len],
        &rows[4][..low_len],
        &rows[5][..low_len],
        &rows[6][..low_len],
        &rows[7][..low_len],
    );
    fill_dense(
        high,
        TWO_INV_K,
        &rows[0][low_len..],
        &rows[1][low_len..],
        &rows[2][low_len..],
        &rows[3][low_len..],
        &rows[4][low_len..],
        &rows[5][low_len..],
        &rows[6][low_len..],
        &rows[7][low_len..],
    );

    lift_dense(low, high, parity == 0, -DELTA);
    lift_dense(high, low, parity == 1, -GAMMA);
    lift_dense(low, high, parity == 0, -BETA);
    lift_dense(high, low, parity == 1, -ALPHA);
    }
    let (first, second) = if parity == 0 { (&*low, &*high) } else { (&*high, &*low) };
    let mut rows = data.chunks_mut(stride);
    let [a, b, c, d, e, f, g, h]: [&mut [i32]; 8] = core::array::from_fn(|_| {
        let row = rows.next().unwrap();
        &mut row[..len]
    });
    scatter_dense(first, second, a, b, c, d, e, f, g, h);
}

#[inline(always)]
fn fill_raw(
    out: &mut [Lane],
    a: &[i32],
    b: &[i32],
    c: &[i32],
    d: &[i32],
    e: &[i32],
    f: &[i32],
    g: &[i32],
    h: &[i32],
) {
    for (target, (((((((&a, &b), &c), &d), &e), &f), &g), &h)) in out
        .iter_mut()
        .zip(a.iter().zip(b).zip(c).zip(d).zip(e).zip(f).zip(g).zip(h))
    {
        *target = [a,b,c,d,e,f,g,h];
    }
}

#[inline(never)]
fn fused_even(low:&mut[Lane],high:&mut[Lane]) {
 let m=high.len();
 let (used,extra)=low.split_at_mut(m);
 let (lhead,ltail)=used.split_at_mut(2);
 let (hhead,htail)=high.split_at_mut(2);
 let [l0,l1]:&mut[Lane;2]=lhead.try_into().unwrap();
 let [h0,h1]:&mut[Lane;2]=hhead.try_into().unwrap();
 let k=splat_f32(K);let hk=splat_f32(TWO_INV_K);
 let cd=splat_f32(-DELTA);let cg=splat_f32(-GAMMA);let cb=splat_f32(-BETA);let ca=splat_f32(-ALPHA);
 let h0_scaled=mul_f32(read_float(h0),hk);
 let h1_scaled=mul_f32(read_float(h1),hk);
 let d0=lift_rows(mul_f32(read_float(l0),k),h0_scaled,h0_scaled,cd);
 let mut dp=lift_rows(mul_f32(read_float(l1),k),h0_scaled,h1_scaled,cd);
 let mut gp=lift_rows(h0_scaled,d0,dp,cg);
 let mut bp=lift_rows(d0,gp,gp,cb);
 write_float(l0,bp);
 let mut hp=h1_scaled;
 let mut ld=l1;let mut hd=h0;let mut hn=h1;
 for (lr,hr) in ltail.iter_mut().zip(htail.iter_mut()) {
  let h=mul_f32(read_float(hr),hk);
  let d=lift_rows(mul_f32(read_float(lr),k),hp,h,cd);
  let g=lift_rows(hp,dp,d,cg);
  let b=lift_rows(dp,gp,g,cb);
  let a=lift_rows(gp,bp,b,ca);
  write_float(ld,b);write_float(hd,a);
  ld=lr;hd=hn;hn=hr;dp=d;gp=g;bp=b;hp=h;
 }
 if let [last]=extra {
  let d=add_f32(mul_f32(read_float(last),k),mul_f32(add_f32(cd,cd),hp));
  let g=lift_rows(hp,dp,d,cg);
  let b=lift_rows(dp,gp,g,cb);
  let a=lift_rows(gp,bp,b,ca);
  let last_b=add_f32(d,mul_f32(add_f32(cb,cb),g));
  let last_a=lift_rows(g,b,last_b,ca);
  write_float(ld,b);write_float(hd,a);write_float(last,last_b);write_float(hn,last_a);
 } else {
  let g=add_f32(hp,mul_f32(add_f32(cg,cg),dp));
  let b=lift_rows(dp,gp,g,cb);
  let a=lift_rows(gp,bp,b,ca);
  let last_a=add_f32(g,mul_f32(add_f32(ca,ca),b));
  write_float(ld,b);write_float(hd,a);write_float(hn,last_a);
 }
}
