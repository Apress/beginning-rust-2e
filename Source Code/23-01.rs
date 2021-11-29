/* ILLEGAL. The compiler prints the error message:
`n` does not live long enough
*/
fn main() {
    let ref_to_n;
    {
        let n = 12;
        ref_to_n = &n;
    }
    print!("{}", *ref_to_n);
}
