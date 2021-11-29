/* It prints:
11*/
fn main() {
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        fn func<'a>(_m1: &'a i32, _m2: &i32) -> &'a i32 {
            _m1
        }
        result = func(&n1, &n2);
    }
    print!("{}", *result);
}
