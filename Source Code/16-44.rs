/* It prints:
0*/
fn main() {
    let s: u32 = [0; 0].into_iter().sum();
    print!("{}", s);
}
