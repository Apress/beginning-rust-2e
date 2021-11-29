/* It prints:
10 16 8 2*/
fn main() {
    let hexadecimal = 0x10;
    let octal = 0o10;
    let binary = 0b10;
    let mut n = 10;
    print!("{} ", n);
    n = hexadecimal;
    print!("{} ", n);
    n = octal;
    print!("{} ", n);
    n = binary;
    print!("{}", n);
}
