/* It prints:
3 + 5 = 8
3.2 + 5.1 = 8.3
*/
fn main() {
    fn print_sum(addend1: f64, addend2: f64) {
        println!("{} + {} = {}", addend1, addend2,
            addend1 + addend2);
    }
    print_sum(3., 5.);
    print_sum(3.2, 5.1);
}
