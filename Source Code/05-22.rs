/* It prints:
3.14, 4, 4.89*/
fn main() {
    let length = 5000;
    let mut y = vec![4.; length];
    y[6] = 3.14;
    y.push(4.89);
    print!("{}, {}, {}", y[6], y[4999], y[5000]);
}
