/* It prints:
3 3*/
fn main() {
    print!("{} {}",
        [1, 2, 3].len(),
        <[i32]>::len(&[1, 2, 3]));
}
