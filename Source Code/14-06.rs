/* It prints:
Hi*/
fn main() {
    let mut a: String = "Xy".to_string(); // "Xy"
    a.remove(0); // "y"
    a.insert(0, 'H'); // "Hy"
    a.pop(); // "H"
    a.push('i'); // "Hi"
    print!("{}", a);
}
