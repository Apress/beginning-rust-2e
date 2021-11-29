/* It prints:
0123456*/
fn main() {
    #[allow(dead_code)]
    enum E {
        Case1(u32),
        Case2(char),
    }
    let mut v = E::Case1(0);
    while let E::Case1(n) = v {
        print!("{}", n);
        if n == 6 { break; }
        v = E::Case1(n + 1);
    }
}
