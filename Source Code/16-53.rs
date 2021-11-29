/* It prints:
[132, 86, 38]*/
fn main() {
    let arr = [66, -8, 43, 19, 0, -31];
    let mut v = vec![];
    for i in 0..arr.len() {
        if arr[i] > 0 { v.push(arr[i] * 2); }
    }
    print!("{:?}", v);
}
