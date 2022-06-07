use std::thread;

static mut A_BAL: u32 = 1000;
static mut B_BAL: u32 = 1000;

fn transfer_A_to_B() {
    let amount = 1;
    A_BAL -= amount;
    B_BAL += amount;
}

fn transfer_B_to_A() {
    let amount = 1;
    B_BAL -= amount;
    A_BAL += amount;
}

fn main() {
    let t1 = thread::spawn(|| { for _ in 0..10000 { transfer_A_to_B(); } } );
    let t2 = thread::spawn(|| { for _ in 0..10000 { transfer_B_to_A(); } } ); 

    t1.join(); t2.join();

    println!("A: {}, B: {}", A_BAL, B_BAL);
}
