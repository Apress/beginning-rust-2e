/* It prints:
brave world */
fn main() {
    let arr = ["hello", "brave", "new", "world"];
    match arr.into_iter().min() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
    match arr.into_iter().max() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
}
