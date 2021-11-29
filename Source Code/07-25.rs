/* It prints:
1234*/
fn main() {
    #[allow(dead_code)]
    enum E {
        Case1(u32),
        Case2(char),
        Case3(i64, bool),
    }
    let v = E::Case3(1234, true);
    if let E::Case3(n, b) = v {
        if b { print!("{}", n) }
    }
}
