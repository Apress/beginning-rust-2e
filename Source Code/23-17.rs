/* ILLEGAL. The compiler prints the two error messages:
explicit lifetime required in the type of `n`
lifetime of reference outlives lifetime of borrowed content...
*/
fn main() {
    fn f(n: &u8) -> &'static u8 { n }
    fn g<'a>(m: &'a u8) -> &'static u8 { m }
}
