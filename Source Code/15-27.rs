/* It prints:
[13, 14] [11, 12, 13]*/
fn main() {
    let arr = [11, 12, 13, 14, 15];
    let r1: std::ops::RangeInclusive<usize> = 2..=3;
    print!("{:?} ", &arr[r1]);
    let r2: std::ops::RangeToInclusive<usize> = ..=2;
    print!("{:?}", &arr[r2]);
}
