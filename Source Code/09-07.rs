/* ILLEGAL. The compiler prints the error message:
cannot find function `f` in this scope
*/
fn main() {
    {
        fn f() { }
    }
    f();
}
