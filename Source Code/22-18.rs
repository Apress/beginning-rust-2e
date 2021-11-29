/* It prints:
123 abc 123s*/
fn main() {
    let i1 = 123;
    let _i2 = i1;
    let s1 = "abc";
    let _s2 = s1;
    let r1 = &i1;
    let _r2 = r1;
    print!("{} {} {}", i1, s1, r1);
}
