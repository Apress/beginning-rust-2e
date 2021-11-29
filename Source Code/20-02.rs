/* It prints:
INNER
Dropped 6
Dropped 5
Dropped 4
OUTER
Dropped 3
Dropped 2
Dropped 1
*/
fn main() {
    struct S ( i32 );
    impl Drop for S {
        fn drop(&mut self) {
            println!("Dropped {}", self.0);
        }
    }
    let _a = S (1);
    let _b = S (2);
    let _c = S (3);
    {
        let _d = S (4);
        let _e = S (5);
        let _f = S (6);
        println!("INNER");
    }
    println!("OUTER");
}
