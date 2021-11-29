/* It prints:
false true */
fn main() {
    print!("{} ", [45, 8, 2, 6].into_iter()
        .any(|n: i32| -> bool { n < 0 }));
    print!("{} ", [45, 8, -2, 6].into_iter()
        .any(|n: i32| -> bool { n < 0 }));
}
