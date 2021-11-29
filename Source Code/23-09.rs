/* ILLEGAL. The compiler prints the error message:
borrow of possibly-uninitialized variable: `_n`
with the explanation:
use of possibly-uninitialized `_n`
*/
fn main() {
    let _n;
    let _r = &_n;
    _n = 12;
}
