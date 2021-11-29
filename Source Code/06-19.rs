/* ILLEGAL. The compiler prints the three error messages:
literal out of range for `i8`
literal out of range for `u16`
literal out of range for `u32`
*/
fn main() {
    let a = 500 as i8;
    let b = 100_000 as u16;
    let c = 10_000_000_000 as u32;
    print!("{} {} {}", a, b, c);    
}
