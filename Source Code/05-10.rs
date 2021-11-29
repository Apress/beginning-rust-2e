/* The compiler prints the warning message:
this operation will panic at runtime
Then, the program panics emitting the message:
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1'
*/
fn main() {
    let x = ["a"];
    #[warn(unconditional_panic)]
    let _y = x[1];
    print!("End");
}
