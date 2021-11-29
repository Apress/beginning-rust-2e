/* It prints:
123 123 123 [] [] File*/
fn main() {
    let a1 = 123;
    let b1 = a1.clone();
    let c1 = b1;
    print!("{} {} {}", a1, b1, c1);
    let a2 = Vec::<bool>::new();
    let b2 = a2.clone();
    let c2 = b2;
    print!(" {:?}", a2);
    // ILLEGAL: print!(" {:?}", b2);
    print!(" {:?}", c2);
    let a3 = std::fs::File::open(".").unwrap();
    // ILLEGAL: let b3 = a3.clone();
    let c3 = a3;
    // ILLEGAL: print!(" {:?}", a3);
    print!(" {:?}", c3);
}
