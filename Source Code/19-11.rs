/* ILLEGAL. The compiler prints the two error messages:
mismatched types
binary operation `==` cannot be applied to type `T`
*/
fn main() {
    fn g(a: i32) { }
    fn f<T>(a: T) -> bool {
        g(a);
        a == a
    }
}
