use std::thread;
use std::sync::atomic::{AtomicI32, Ordering};

static A_BAL: AtomicI32 = AtomicI32::new(1000);
static B_BAL: AtomicI32 = AtomicI32::new(1000);


fn transfer_a_to_b() {
    let amount = 1;
    A_BAL.fetch_sub(amount, Ordering::Relaxed);
    B_BAL.fetch_add(amount, Ordering::Relaxed);
}

fn transfer_b_to_a() {
    let amount = 1;
    B_BAL.fetch_sub(amount, Ordering::Relaxed);
    A_BAL.fetch_add(amount, Ordering::Relaxed);
}

fn main() {
    let t1 = thread::spawn(|| { for _ in 0..100000 { transfer_a_to_b(); } } );
    let t2 = thread::spawn(|| { for _ in 0..100000 { transfer_b_to_a(); } } );

    let (_, _) = (t1.join(), t2.join());

    println!("A: {}, B: {}", A_BAL.load(Ordering::Relaxed), B_BAL.load(Ordering::Relaxed));
}
