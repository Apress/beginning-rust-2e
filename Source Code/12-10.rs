/* It prints:
160 16 1600 1 16 24*/
#[allow(dead_code)]
fn main() {
    enum E1 { E1a, E1b }
    enum E2 { E2a, E2b(f64) }
    use std::mem::*;
    print!("{} {} {} {} {} {}",
        size_of_val(&[0i16; 80]),
        size_of_val(&(0i16, 0i64)),
        size_of_val(&[(0i16, 0i64); 100]),
        size_of_val(&E1::E1a),
        size_of_val(&E2::E2a),
        size_of_val(&vec![(0i16, 0i64); 100]));
}
