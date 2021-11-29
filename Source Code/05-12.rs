/* It prints:
This is a nice sentence.*/
fn main() {
    let mut x = ["This", "is", "a", "sentence"];
    x[2] = "a nice";
    print!("{} {} {} {}.", x[0], x[1], x[2], x[3]);
}
