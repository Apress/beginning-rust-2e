/* ILLEGAL. The compiler prints the three error messages:
the type `[{integer}]` cannot be indexed by `isize`
the type `[{integer}]` cannot be indexed by `u32`
the type `[{integer}]` cannot be indexed by `u64`
*/
fn main() {
    let arr = [11, 22, 33];
    let i: usize = 2;
    print!("{}", arr[i]);
    let i: isize = 2;
    print!("{}", arr[i]);
    let i: u32 = 2;
    print!("{}", arr[i]);
    let i: u64 = 2;
    print!("{}", arr[i]);
}
