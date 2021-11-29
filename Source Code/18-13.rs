/* It prints:
123*/
fn main() {
    mod routines {
        pub fn f() -> u32 { g() }
        fn g() -> u32 { 123 }
    }
    print!("{}", routines::f());
}
