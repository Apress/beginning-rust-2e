/* It prints:
7 7; 7 9 9*/
fn main() {
    let a = 7;
    let a_box: Box<i32>;
    let mut a_ref: &i32 = &a;
    print!("{} {};", a, *a_ref);
    a_box = Box::new(a + 2);
    a_ref = &*a_box;
    print!(" {} {} {}", a, *a_ref, *a_box);
}
