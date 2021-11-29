/* It prints:
3.4 [1, 2, 3] 3.4 true*/
fn main() {
    fn f(p: &f64) {
        let a = Box::new(*p);
        {
            let b = Box::new([1, 2, 3]);
            print!("{} {:?}", *a, *b);
        }
        let c = Box::new(true);
        print!(" {} {}", a, c);
    }
    f(&3.4);
}
