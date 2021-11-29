/* It prints:
2.3 3.4*/
fn main() {
    fn f1(x: f32) -> f32 { x }
    fn f2(x: f32) -> f32 { x }
    let a: f32 = 2.3;
    let b: f32 = 3.4;
    print!("{} {}", f1(a), f2(b));
}
