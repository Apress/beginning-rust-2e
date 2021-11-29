/* ILLEGAL. The compiler prints the error message:
use of moved value: `v1`
*/
fn main() {
    let v1 = vec![false; 3];
    let mut _v2 = vec![false; 2];
    _v2 = v1;
    v1;
}
