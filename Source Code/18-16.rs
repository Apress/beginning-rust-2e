/* It prints:
2.3 3.4*/
fn main() {
    type Number = f32;
    fn f1(x: Number) -> Number { x }
    fn f2(x: Number) -> Number { x }
    let a: Number = 2.3;
    let b: Number = 3.4;
    print!("{} {}", f1(a), f2(b));
}
