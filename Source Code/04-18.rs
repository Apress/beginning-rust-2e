/* It prints:
3 1, 2 2, 1 3, 0 4, -1 5, -1*/
fn main() {
    let mut limit = 4;
    for n in 1..limit + 2 {
        limit -= 1;
        print!("{} {}, ", limit, n);
    }
    print!("{}", limit);
}
