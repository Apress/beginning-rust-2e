/* ILLEGAL. The compiler prints the error message:
use of moved value: `v1`
*/
fn main() {
    fn f(_v2: Vec<bool>) {}
    let v1 = vec![false; 3];
    f(v1);
    v1;
}
