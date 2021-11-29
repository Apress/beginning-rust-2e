/* It prints:
abc. XYZ. 123.*/
fn main() {
    let mut x = ["a", "b", "c"];
    print!("{}{}{}. ", x[0], x[1], x[2]);
    x = ["X", "Y", "Z"];
    print!("{}{}{}. ", x[0], x[1], x[2]);
    let y = ["1", "2", "3"];
    x = y;
    print!("{}{}{}.", x[0], x[1], x[2]);
}
