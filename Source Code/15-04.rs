/* It prints:
2 2 2 8 8 16*/
fn main() {
    let r1 = 3u8..12u8;
    let r2 = 3u8..12;
    let r3 = 3..12u8;
    let r4 = 3..12;
    let r5 = -3..12;
    let r6 = 3..12 as i64;
    print!(
        "{} {} {} {} {} {}",
        std::mem::size_of_val(&r1),
        std::mem::size_of_val(&r2),
        std::mem::size_of_val(&r3),
        std::mem::size_of_val(&r4),
        std::mem::size_of_val(&r5),
        std::mem::size_of_val(&r6));
}
