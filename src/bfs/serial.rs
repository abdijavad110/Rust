use super::*;

use std::time::Instant;
use std::hint::spin_loop;
use std::collections::VecDeque;

struct Node<V> {
    children: Vec<Node<V>>,
    value: V,
}

impl<V> Node<V> {
    fn bfs(&self, f: impl Fn(&V)) {
        let mut q = VecDeque::new();
        q.push_back(self);

        while let Some(t) = q.pop_front() {
            (f)(&t.value);
            (&t.children).into_iter().for_each(|child| q.push_back(child));
        }
    }
}

fn prefect_int_bin_tree(depth: usize) -> Node<i32> {
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
    root.bfs(|_| spin(conf::SPIN_DELAY));
    // root.bfs(|q| print!("{}|", q));
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