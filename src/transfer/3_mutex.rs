use std::thread;
use std::sync::{Arc, Mutex};


fn transfer_a_to_b(src: &Arc<Mutex<i32>>, dest: &Arc<Mutex<i32>>) {
    let mut src_bal = src.lock().unwrap();
    let mut dest_bal = dest.lock().unwrap();
    
    let amount = 1;
    *src_bal -= amount;
    *dest_bal += amount;
}

fn transfer_b_to_a(src: &Arc<Mutex<i32>>, dest: &Arc<Mutex<i32>>) {
    let mut dest_bal = dest.lock().unwrap();
    let mut src_bal = src.lock().unwrap();

    let amount = 1;
    *src_bal -= amount;
    *dest_bal += amount;
}

fn main() {
    let bal_a = Arc::new(Mutex::new(1000i32));
    let la1 = Arc::clone(&bal_a);
    let la2 = Arc::clone(&bal_a);

    let bal_b = Arc::new(Mutex::new(1000i32));
    let lb1 = Arc::clone(&bal_b);
    let lb2 = Arc::clone(&bal_b);

    let t1 = thread::spawn(move || { for _ in 0..100000 { transfer_a_to_b(&la1, &lb1); } } );
    let t2 = thread::spawn(move || { for _ in 0..100000 { transfer_b_to_a(&lb2, &la2); } } ); 

    let (_, _) = (t1.join(), t2.join());

    println!("A: {}, B: {}", *(bal_a.lock().unwrap()), *(bal_b.lock().unwrap()));
}
