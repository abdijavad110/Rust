// https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/mandelbrot-rust-1.html
use super::*;

use std::io::Write;
use std::thread;
const THREADS: usize = 1;
const MAX_ITER: usize = 50;
const VLEN: usize = 8;
const ZEROS: Vecf64 = [0.0; VLEN];

pub type Vecf64 = [f64; VLEN];

fn mul2 (x: Vecf64, y: Vecf64) -> Vecf64 {
    let mut res = ZEROS;
    for i in 0..VLEN { res[i] = x[i] * y[i]; }
    res
}
fn add2 (x: Vecf64, y: Vecf64) -> Vecf64 {
    let mut res = ZEROS;
    for i in 0..VLEN { res[i] = x[i] + y[i]; }
    res
}
fn sub2 (x: Vecf64, y: Vecf64) -> Vecf64 {
    let mut res = ZEROS;
    for i in 0..VLEN { res[i] = x[i] - y[i]; }
    res
}

pub fn mbrot8(cr: Vecf64, ci: Vecf64) -> u8 {
    let mut zr = cr;
    let mut zi = ci;
    let mut esc_bits = 0;
    for _ in 0..MAX_ITER {
        // Find Re(z)^2 and Im(z)^2
        let rr  = mul2(zr,zr);
        let ii  = mul2(zi,zi);
        // Check if we escape
        // May as well store this info in
        // same byte as output
        let mag = add2(rr, ii);
        for i in 0..VLEN {
            if mag[i] > 4.0 { esc_bits |= 128 >> i; }
        }
        // If no more work, break early
        if esc_bits == 0xff { break; }
        // Find Im(z^2)
        let ir = mul2(zr, zi);
        // Set Re(z^2)
        zr = sub2(rr, ii);
        // Set Im(z^2)
        zi = add2(ir, ir);
        // Add c
        zr = add2(zr, cr);
        zi = add2(zi, ci);
    }
    !esc_bits
}

fn run(n_inp: i32) {
    let size = n_inp as usize;
    let inv = 2.0 / size as f64;
    let mut xvals = vec![0.0; size];
    let mut yvals = vec![0.0; size];
    for i in 0..size {
        xvals[i] = i as f64 * inv - 1.5;
        yvals[i] = i as f64 * inv - 1.0;
    }
    let xloc = &xvals;
    let yloc = &yvals;

    let xloc = xloc.to_vec();
    let yloc = yloc.to_vec();
    let mut rows = vec![vec![0 as u8; size / 8]; size];
    for y in 0..size {
        for x in 0..size / 8 {
            let mut cr = ZEROS;
            let ci = [yloc[y]; VLEN];
            for i in 0..VLEN {
                cr[i] = xloc[8 * x + i];
            }
            rows[y][x] = mbrot8(cr, ci);
        }
    }
}

#[cfg(test)]
mod benches {
    use super::*;

    #[bench]
    fn small(b: &mut Bencher) { b.iter(|| { run(S_N) }); }
    #[bench]
    fn medium(b: &mut Bencher) { b.iter(|| { run(M_N) }); }
    #[bench]
    fn large(b: &mut Bencher) { b.iter(|| { run(L_N) }); }
}
