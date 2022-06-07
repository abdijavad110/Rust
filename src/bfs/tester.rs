use super::*;

use super::csr;
use super::tree::Node;
use std::time::Instant;
use std::hint::spin_loop;

static mut G_ID: u32 = 0;


fn prefect_int_bin_tree(depth: usize) -> Node {
    let mut children = vec![];
    if depth != 1 {
        children.push(prefect_int_bin_tree(depth - 1));
        children.push(prefect_int_bin_tree(depth - 1));
    }
    unsafe { G_ID += 1; }
    Node { id: unsafe { G_ID }, distance: u32::MAX, children, value: depth as i32 }
}

fn spin(n: usize) {
    for _ in 0..n { spin_loop(); }
}

pub fn run_tree() {
    let mut root = prefect_int_bin_tree(conf::M_N as usize);
    root.distance = 0;

    let start_time = Instant::now();
    root.pbfs_semi(|_| spin(conf::SPIN_DELAY));
    // root.pbfs_semi(|q| print!("{}|", q));
    let duration = start_time.elapsed();

    println!("finieshed execution in {:?}", duration);
}

static BASE: usize = 2;
pub static mut DISTANCES: Vec<usize> = Vec::new();

pub fn run_csr() {
    let depth = conf::M_N as usize;
    let max = BASE.pow(depth as u32);
    let mut idxs:Vec<usize> = vec![0; max];
    let mut adjs:Vec<usize> = Vec::new();

    unsafe { DISTANCES = vec![usize::MAX; max-1]; DISTANCES[0] = 0; };


    for i in 0..max {
        idxs[i] = adjs.len() as usize;
        if i < (BASE.pow((depth - 1) as u32)) - 1 {
            adjs.push(2 * i as usize + 1);
            adjs.push(2 * i as usize + 2);
        }
    }

    // println!("indices: {:?}", idxs);
    // println!("adjacency: {:?}", adjs);

    let start_time = Instant::now();
    // let distances = csr::bfs(&idxs, &adjs);
    csr::pbfs_tm(&idxs, &adjs);
    let duration = start_time.elapsed();

    // println!("distances: {:?}", distances);
    unsafe { println!("min_dist: {:?}, max_dist: {:?}", DISTANCES[0], DISTANCES[DISTANCES.len() - 1]); }
    println!("finieshed execution in {:?}", duration);
}
