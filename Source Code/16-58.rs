/* It prints:
F66 M66 F-8 F43 M43 F19 M19 F0 F-31 [132, 86, 38]*/
fn main() {
    let mut v = vec![];
    for item in [66, -8, 43, 19, 0, -31]
        .into_iter()
        .filter(|x| { print!("F{} ", x); *x > 0 })
        .map(|x| { print!("M{} ", x); x * 2 }) {
        v.push(item);
    }
    print!("{:?}", v);
}
