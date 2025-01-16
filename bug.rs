fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; 
    *y += 1; 
    *z += 1; // Data race! 
    println!("x = {}", x);
}