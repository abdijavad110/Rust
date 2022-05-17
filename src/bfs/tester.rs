use super::*;

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

pub fn run() {
    let mut root = prefect_int_bin_tree(conf::M_N as usize);
    root.distance = 0;

    let start_time = Instant::now();
    // root.bfs(|_| spin(conf::SPIN_DELAY));
    root.bfs(|q| print!("{}|", q));
    let duration = start_time.elapsed();

    println!("finieshed execution in {:?}", duration);
}

