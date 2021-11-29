/* It prints:
Hello, world!*/
fn main() {
    let comma = ", ".to_string();
    let world = "world".to_string();
    let excl_point = '!';
    let mut result = "Hello".to_string();
    result += &comma;
    result.push_str(&world);
    result.push(excl_point);
    print!("{}", result);
}
