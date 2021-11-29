/* It prints:
[132, 86, 38]*/
fn main() {
    let arr = [66, -8, 43, 19, 0, -31];
    let mut v = vec![];
    for n in arr.into_iter() {
        if n > 0 { v.push(n * 2); }
    }
    print!("{:?}", v);
}
