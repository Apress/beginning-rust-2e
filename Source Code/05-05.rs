/* ILLEGAL. The compiler prints the 5 error messages:
mismatched types
the trait bound `usize: Neg` is not satisfied
the type `[&str]` cannot be indexed by `{float}`
the type `[&str]` cannot be indexed by `bool`
the type `[&str]` cannot be indexed by `&str`
*/
fn main() {
    let mut x = ["a"]; // array of strings
    x[0] = 3;
    x[-1] = "b";
    x[0.] = "b";
    x[false] = "b";
    x["0"] = "b";
}
