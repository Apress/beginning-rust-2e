/* The compiler prints three warnings,
and then the program prints nothing.
*/
fn main() {
    fn f<T>(a: T) -> T { a }
    fn g<T>(a: T) -> T {
        let b: T = a;
        let mut c = b;
        c = f(c);
        c
    }
    fn h<T>(a: &T) -> &T { a }
}
