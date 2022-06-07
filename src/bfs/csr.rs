use rayon::prelude::*;
use std::arch::x86_64::{_xbegin, _xabort, _xend};

pub fn bfs(idxs: &Vec<usize>, adjs: &Vec<usize>) -> Vec<usize> {
    let mut depth = 0usize;
    let mut q:Vec<usize> = Vec::new();
    let mut distances = vec![usize::MAX; idxs.len() - 1];
    q.push(0usize);
    distances[0] = depth;

    while !q.is_empty() {
        depth += 1;
        let children:Vec<usize> = (&q).iter().map(|v| {
            (idxs[*v]..idxs[*v+1]).filter_map(|c| {
                if distances[adjs[c]] == usize::MAX { Some(adjs[c]) }
                else { None }
            }).collect::<Vec<usize>>()
        }).flatten().collect::<Vec<usize>>();
        (&children).iter().for_each(|v| distances[*v] = depth);
        q.clear();
        q.extend(children);
    }
    distances
}

pub fn pbfs_tm(idxs: &Vec<usize>, adjs: &Vec<usize>) {
    let mut depth = 0usize;
    let mut q:Vec<usize> = Vec::new();
    q.push(0usize);

    while !q.is_empty() {
        depth += 1;
        println!("processing depth={}", depth);
        let children:Vec<usize> = (&q).par_iter().map(|v| {
            loop {
                if unsafe { _xbegin() } == 0xffffffff {
                    let cs = (idxs[*v]..idxs[*v + 1]).filter_map(|ci| {
                        let c = adjs[ci];
                        if unsafe { super::tester::DISTANCES[c] } == usize::MAX {
                            unsafe { super::tester::DISTANCES[c] = depth };
                            Some(c)
                        } else { None }
                    }).collect::<Vec<usize>>();
                    unsafe { _xend(); }
                    return cs;
                }
                else {
                    unsafe { _xabort(0xff); }
                    std::thread::sleep(std::time::Duration::from_millis(25));
                }
            }
        }).flatten().collect::<Vec<usize>>();
        q.clear();
        q.par_extend(children);
    }
}

pub fn pbfs_unsafe(idxs: &Vec<usize>, adjs: &Vec<usize>) {
    let mut depth = 0usize;
    let mut q:Vec<usize> = Vec::new();
    q.push(0usize);

    while !q.is_empty() {
        depth += 1;
        let children:Vec<usize> = (&q).par_iter().map(|v| {

            (idxs[*v]..idxs[*v+1]).filter_map(|ci| {
                let c = adjs[ci];
                if unsafe { super::tester::DISTANCES[c] } == usize::MAX {
                    unsafe { super::tester::DISTANCES[c] = depth };
                    Some(c)
                } else { None }
            }).collect::<Vec<usize>>()

        }).flatten().collect::<Vec<usize>>();
        q.clear();
        q.par_extend(children);
    }
}

pub fn pbfs_safe(idxs: &Vec<usize>, adjs: &Vec<usize>) -> Vec<usize> {
    let mut depth = 0usize;
    let mut q:Vec<usize> = Vec::new();
    let mut distances = vec![usize::MAX; idxs.len() - 1];
    q.push(0usize);
    distances[0] = depth;

    while !q.is_empty() {
        depth += 1;
        let children:Vec<usize> = (&q).par_iter().map(|v| {
            (idxs[*v]..idxs[*v+1]).filter_map(|c| {
                if distances[adjs[c]] == usize::MAX { Some(adjs[c]) } else { None }
            }).collect::<Vec<usize>>()
        }).flatten().collect::<Vec<usize>>();
        (&children).iter().for_each(|v| distances[*v] = depth );
        q.clear();
        q.extend(children);
    }
    distances
}