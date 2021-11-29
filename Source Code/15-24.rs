/* It prints:
3.. ..12 4 4*/
fn main() {
    let r1: std::ops::RangeFrom<i32> = 3..;
    let r2: std::ops::RangeTo<i32> = ..12;
    print!("{:?} {:?} {} {}", r1, r2,
        std::mem::size_of_val(&r1),
        std::mem::size_of_val(&r2));
}
