/* It prints:
-2 45 ---*/
fn main() {
    let arr = [45, 8, -2, 6];
    match arr.into_iter().min() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
    match arr.into_iter().max() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
    match [0; 0].into_iter().min() {
        Some(n) => print!("{} ", n),
        _ => print!("---"),
    }
}
