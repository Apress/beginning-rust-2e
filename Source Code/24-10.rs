/* ILLEGAL. The compiler prints the error message:
`x` does not live long enough
*/
fn main() {
    let y: &i32;
    {
        let x: i32 = 12;
        y = &x;
    }
    print!("{}", *y);
}
