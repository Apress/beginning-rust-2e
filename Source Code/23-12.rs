/* It prints:
11*/
fn main() {
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        result = {
            let _m1 = &n1;
            let _m2 = &n2;
            _m1
        }
    }
    print!("{}", *result);
}
