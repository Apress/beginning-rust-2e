/* It prints:
[132, 86, 38]*/
fn main() {
    let arr = [66, -8, 43, 19, 0, -31];
    let v = arr
        .into_iter()
        .filter(|x| *x > 0)
        .map(|x| x * 2)
        .collect::<Vec<_>>();
    print!("{:?}", v);
}
