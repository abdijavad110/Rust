use std::collections::{HashSet, VecDeque};
use rayon::prelude::*;


#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Node {
    pub id: u32,
    pub distance: u32,
    pub children: Vec<Node>,
    pub value: i32,
}

impl Node {
    pub fn bfs(&mut self, f: impl Fn(i32)) {
        let mut q = VecDeque::new();
        q.push_back(self);

        while let Some(v) = q.pop_front() {
            (f)(v.distance as i32);
            (&mut v.children).iter_mut().for_each(|child| {
                child.distance = v.distance + 1;
                q.push_back(child)
            });
        }
    }

    pub fn pbfs(&mut self, f: impl Fn(i32) + std::marker::Sync) {
        let mut q:HashSet<&Node> = HashSet::new();
        q.insert(self);

        while !q.is_empty() {
            q = (&q).into_par_iter().map(
                |v| {
                    (f)(v.distance as i32);
                    &v.children
                }
            ).flatten().collect::<HashSet<&Node>>();
        }
    }
}
