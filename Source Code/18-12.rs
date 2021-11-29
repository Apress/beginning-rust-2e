/* ILLEGAL. The compiler prints the error message:
cannot find function `f` in this scope
*/
fn main() {
    {
        fn f() -> u32 { g() }
        fn g() -> u32 { 123 }
    }
    print!("{}", f());
}
