/* ILLEGAL. The compiler prints the error message:
`n2` does not live long enough
*/
fn main() {
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        fn func<'a>(_m1: &i32, _m2: &'a i32) -> &'a i32 {
            _m2
        }
        result = func(&n1, &n2);
    }
    print!("{}", *result);
}
