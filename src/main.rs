#![feature(test)]
#![feature(stdsimd)]

extern crate rayon;

#[allow(unstable_features)]
#[allow(soft_unstable)]
#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]
#[allow(non_snake_case)]

// mod binary_trees;
// mod mandelbrot;
mod bfs;

use bfs::tester::run_csr;
// use bfs::tester::run_tree;

fn main() {
    run_csr()
}

// fn main() -> Result<(), &'static str> {
//     Err("Incorrect use. Run `cargo bench`.\n\
//     The size of benchmarks could be specified with `-- [small/medium/large]`")
// }