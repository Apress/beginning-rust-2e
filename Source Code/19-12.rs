/* It prints:
A, B; B, A*/
fn main() {
    let mut a = 'A';
    let mut b = 'B';
    print!("{}, {}; ", a, b);
    std::mem::swap(&mut a, &mut b);
    print!("{}, {}", a, b);
}
