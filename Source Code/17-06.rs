/* It prints:
[Err(NotPresent)] [Ok("This is the value")]*/
fn main() {
    print!("[{:?}]", std::env::var("abcd"));
    std::env::set_var("abcd", "This is the value");
    print!(" [{:?}]", std::env::var("abcd"));
}
