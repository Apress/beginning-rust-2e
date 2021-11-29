/* ILLEGAL. The compiler prints the error message:
type annotations needed
*/
fn main() {
    print!(
        "{}",
        (|v| {
            let mut sum = 0;
            for i in 0..v.len() {
                sum += v[i];
            }
            sum
        })(&vec![11, 22, 34]));
}
