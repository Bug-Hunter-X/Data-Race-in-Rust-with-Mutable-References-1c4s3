use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));
    
    let y = x.clone();
    let z = x.clone();

    let handle1 = std::thread::spawn(move || {
        let mut x_mut = y.lock().unwrap();
        *x_mut += 1;
    });

    let handle2 = std::thread::spawn(move || {
        let mut x_mut = z.lock().unwrap();
        *x_mut += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("x = {}", *x.lock().unwrap());
}
