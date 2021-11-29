/* ILLEGAL. The compiler prints the error message:
type annotations needed for `[_; 0]`
with the explanation:
cannot infer type
*/
fn main() {
    let _a = [];
}
