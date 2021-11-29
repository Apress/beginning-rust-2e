/* If there is the file created by the previous program,
this program prints:
eè€
*/
fn main() {
    use std::io::Read;
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
