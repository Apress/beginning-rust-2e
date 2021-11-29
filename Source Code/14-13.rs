/* It prints:
Hello Hello Hello Hello */
fn main() {
    let ss1 = "He";
    let ss2 = "llo ";
    let ds1 = ss1.to_string();
    let ds2 = ss2.to_string();
    let ds3 = format!("{}{}", ss1, ss2);
    print!("{}", ds3);
    let ds3 = format!("{}{}", ss1, ds2);
    print!("{}", ds3);
    let ds3 = format!("{}{}", ds1, ss2);
    print!("{}", ds3);
    let ds3 = format!("{}{}", ds1, ds2);
    print!("{}", ds3);
}
