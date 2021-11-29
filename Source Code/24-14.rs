/* It prints:
true 1*/
// In some library code:
struct S { b: bool, ri: &'static i32 }
fn create_s(ri: &i32) -> S {
    static ZERO: i32 = 0;
    static ONE: i32 = 1;
    S {
        b: true,
        ri: if *ri > 0 { &ONE } else { &ZERO },
    }
}
// In application code:
fn main() {
    let y: S;
    {
        let x: i32 = 12;
        y = create_s(&x);
    }
    print!("{} {}", y.b, *y.ri);
}
