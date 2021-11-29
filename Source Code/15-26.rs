/* It prints:
0 [11, 22, 33, 44] [11, 22, 33, 44]*/
fn main() {
    let range: std::ops::RangeFull = ..;
    let a1 = [11, 22, 33, 44];
    let a2 = &a1[range];
    print!("{} {:?} {:?}", std::mem::size_of_val(&range), a1, a2);
}
