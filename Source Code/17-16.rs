/* It prints:
Hello world*/
fn main() {
    use std::io::Write;
    //ILLEGAL: std::io::stdout().write("Hi").unwrap();
    //ILLEGAL: std::io::stdout().write(String::from("Hi")).unwrap();
    std::io::stdout().write("Hello ".as_bytes()).unwrap();
    std::io::stdout().write(String::from("world").as_bytes()).unwrap();
}
