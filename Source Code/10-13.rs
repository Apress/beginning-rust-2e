/* It prints:
(4.5, 'A') (-6.0, 'g')*/
fn main() {
    fn swap<T1, T2>(a: T1, b: T2) -> (T2, T1) { (b, a) }
    let x = swap('A', 4.5);
    let y = swap('g', -6.);
    print!("{:?} {:?}", x, y);
}
