use super::*;

extern crate rayon;

use std::time::Instant;
use std::hint::spin_loop;
use std::collections::VecDeque;
use self::rayon::prelude::*;

struct Node {
    children: Vec<Node>,
    value: i32,
}

impl Node {
    fn bfs(&self, f: impl Fn(i32) + std::marker::Sync) {
        let mut q = VecDeque::new();
        q.push_back(self);

        let mut children = VecDeque::new();

        while !q.is_empty() {
            (&q).into_par_iter().for_each(|v| (f)(v.value));
            children = (&q).into_par_iter().map(|v| &v.children).flatten().collect::<VecDeque<&Node>>();
            q.clear();
            children.into_iter().for_each(|c| q.push_back(c));
        }
    }
}


fn prefect_int_bin_tree(depth: usize) -> Node {
    let mut children = vec![];
    if depth != 1 {
        children.push(prefect_int_bin_tree(depth - 1));
        children.push(prefect_int_bin_tree(depth - 1));
    }
    Node { children, value: depth as i32 }
}

fn spin(n: usize) {
    for _ in 0..n {spin_loop();}
}

pub fn run(n: i32) {
    let root = prefect_int_bin_tree(n as usize);

    let start_time = Instant::now();
    root.bfs(|_| spin(10000));
    // root.bfs(|q| print!("{}|", n - q));
    let duration = start_time.elapsed();

    println!("finieshed execution in {:?}", duration);
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