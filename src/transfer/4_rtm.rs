#![feature(stdsimd)]

use std::thread;
use std::arch::x86_64::{_xabort, _xbegin, _xend};

static mut A_BAL: i32 = 1000;
static mut B_BAL: i32 = 1000;

fn transfer_a_to_b() {
    let amount = 1;
    loop {
        if unsafe { _xbegin() } == 0xFFFFFFFF {
            unsafe { A_BAL -= amount; B_BAL += amount; }
            unsafe { _xend(); }
            break;
        }
        else { unsafe { _xabort(0xff); } }
    }
}

fn transfer_b_to_a() {
    let amount = 1;
    loop {
        if unsafe { _xbegin() } == 0xFFFFFFFF {
            unsafe { B_BAL -= amount; A_BAL += amount; }
            unsafe { _xend(); }
            break;
        }
        else { unsafe { _xabort(0xff); } }
    }
}

fn main() {
    let t1 = thread::spawn(|| { for _ in 0..100000 { transfer_a_to_b(); } } );
    let t2 = thread::spawn(|| { for _ in 0..100000 { transfer_b_to_a(); } } );

    let (_, _) = (t1.join(), t2.join());

    unsafe { println!("A: {}, B: {}", A_BAL, B_BAL); }
}
