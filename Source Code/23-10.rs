/* ILLEGAL. The compiler prints the error message:
`_n` does not live long enough
*/
fn main() {
    let _r;
    {
        let _n = 12;
        _r = &_n;
    }
    print!("{}", _r);
}
