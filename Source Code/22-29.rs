/* It prints:
13 12*/
fn main() {
    struct S { x: Vec<i32> }
    impl Clone for S {
        fn clone(&self) -> Self {
            S { x: self.x.clone() }
        }
    }
    let mut s1 = S { x: vec![12] };
    let s2 = s1.clone();
    s1.x[0] += 1;
    print!("{} {}", s1.x[0], s2.x[0]);
}
