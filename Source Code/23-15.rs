/* ILLEGAL. The compiler prints the error message:
missing lifetime specifier
*/
fn main() {
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        fn func(_m1: &i32, _m2: &i32) -> &i32 {
            _m2
        }
        result = func(&n1, &n2);
    }
    print!("{}", *result);
}
