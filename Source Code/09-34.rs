/* It prints:
7 7 7 7*/
fn main() {
    let a = &&&7;
    print!("{} {} {} {}", ***a, **a, *a, a);
}
