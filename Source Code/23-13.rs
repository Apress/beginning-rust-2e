/* ILLEGAL. The compiler prints the error message:
`n2` does not live long enough
*/
fn main() {
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        result = {
            let _m1 = &n1;
            let _m2 = &n2;
            _m2
        }
    }
    print!("{}", *result);
}
