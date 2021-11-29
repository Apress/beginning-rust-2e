/* It prints:
45 4.5 true*/
fn main() {
    let int_str: String = 45.to_string();
    let float_str: String = 4.5.to_string();
    let bool_str: String = true.to_string();
    print!("{} {} {}", int_str, float_str, bool_str);
}
