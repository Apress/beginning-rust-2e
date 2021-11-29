/* ILLEGAL. The compiler prints three times the error message:
the size for values of type `str` cannot be known at compilation time
*/
fn main() {
    let a: str;
    fn f(a: str) {}
    print!("{}", std::mem::size_of::<str>());
}
