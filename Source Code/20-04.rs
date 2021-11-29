/* It prints:
Dropped 1
Dropped 2
Dropped 3
Dropped 4
Dropped 5
Dropped 6
INNER
OUTER
*/
fn main() {
    struct S ( i32 );
    impl Drop for S {
        fn drop(&mut self) {
            println!("Dropped {}", self.0);
        }
    }
    S (1); S (2); S (3);
    {
        S (4); S (5); S (6);
        println!("INNER");
    }
    println!("OUTER");
}
