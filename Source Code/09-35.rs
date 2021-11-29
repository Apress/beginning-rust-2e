/* It prints:
10 11 20 21 */
fn main() {
    let mut a: i32 = 10;
    let mut b: i32 = 20;
    let mut p: &mut i32 = &mut a; // line 3
    print!("{} ", *p);
    *p += 1; // line 5
    print!("{} ", *p);
    p = &mut b; // line 7
    print!("{} ", *p);
    *p += 1; // line 9
    print!("{} ", *p);
}
