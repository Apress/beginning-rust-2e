/* It prints:
-12 34464 1410065408*/
#[allow(overflowing_literals)]
fn main() {
    let a = 500 as i8;
    let b = 100_000 as u16;
    let c = 10_000_000_000 as u32;
    print!("{} {} {}", a, b, c);    
}
