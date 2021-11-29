/* It prints:
(4, 6)*/
fn main() {
    fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
        (dividend / divisor, dividend % divisor)
    }
    print!("{:?}", divide(50, 11));
}
