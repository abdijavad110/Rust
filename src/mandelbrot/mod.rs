extern crate test;
use test::Bencher;

mod serial;
mod multi_thread;
mod rayon;

mod conf;
use conf::{S_N, M_N, L_N};
