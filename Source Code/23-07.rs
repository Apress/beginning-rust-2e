/* It prints:
12 12 23*/
fn main() {
    let a = 12;
    let mut b = &a; // start of first borrow of a to b
    let c = &a; // start of borrow of a to c
    print!("{} {} ", b, c); // end of borrows of a to b and c
    b = &23; // start of second borrow of a to b
    print!("{}", b); // end of second borrow of a to b    
}
