/* The compiler prints the warning message:
unused `Map` that must be used
and then the note:
iterators are lazy and do nothing unless consumed
Then, the program prints nothing.
*/
fn main() {
    [66, -8, 43, 19, 0, -31]
        .into_iter()
        .filter(|x| { print!("F{} ", x); *x > 0 })
        .map(|x| { print!("M{} ", x); x * 2 });
}
