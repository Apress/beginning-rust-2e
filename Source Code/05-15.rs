/* It prints:
4, 3.14*/
fn main() {
    let mut x = [4.; 5000];
    x[2000] = 3.14;
    print!("{}, {}", x[1000], x[2000]);
}
