use std::collections::{HashSet, VecDeque};
use rayon::prelude::*;
use std::sync::Mutex;


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
            (f)(v.value);
            (&mut v.children).iter_mut().for_each(|child| {
                child.distance = v.distance + 1;
                q.push_back(child)
            });
        }
    }

    pub fn pbfs_wrong(&mut self, f: impl Fn(i32) + std::marker::Sync) {
        let mut q:HashSet<&Node> = HashSet::new();
        q.insert(self);

        while !q.is_empty() {
            q = (&q).into_par_iter().map(
                |v| {
                    (f)(v.value);
                    &v.children
                }
            ).flatten().collect::<HashSet<&Node>>();
        }
    }

    pub fn pbfs_semi(&mut self, _f: impl Fn(i32) + std::marker::Sync) {
        // let mut q:Vec<&mut Node> = Vec::new();
        // q.push(self);
        //
        // loop {
        //     let mut new_q = Vec::new();
        //     for v in q {
        //         (f)(v.value);
        //         for mut c in v.children {
        //             c.distance = v.distance + 1;
        //             new_q.push(&mut c);
        //         }
        //     }
        //     q = new_q;
        // }

        // let mut q:Vec<&mut Node> = Vec::new();
        // while !q.is_empty() {
        //     q = (&q).into_iter().map(
        //         |v| {
        //             (f)(v.value);
        //             &v.children
        //         }
        //     ).flatten().collect::<Vec<&mut Node>>();
        // }
    }

    // pub fn pbfs_semi(&mut self, f: impl Fn(i32) + std::marker::Sync) {
    //     let mut q:Vec<&mut Node> = Vec::new();
    //     q.push(self);
    //
    //     while !q.is_empty() {
    //         let mut new_q:Vec<&mut Node> = Vec::new();
    //         let new_q_lock = Mutex::new(new_q);
    //         (&q).par_iter_mut().for_each(
    //             |v| {
    //                 (f)(v.value);
    //                 v.children.iter_mut().for_each(|c| c.distance = v.distance + 1);
    //                 let new_q = new_q_lock.lock().unwrap();
    //                 new_q.extend(&mut v.children);
    //             }
    //         );
    //         let new_q = new_q_lock.lock().unwrap().to_vec();
    //         q = new_q;
    //     }
    // }

    // TODO: write unsafe version
}
