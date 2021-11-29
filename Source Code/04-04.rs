/* It prints:
small*/
fn main() {
    let n = 4;
    if n > 1000 {
        print!("big");
    }
    else if n > 0 {
        print!("small");
    }
    else if n < 0 {
        print!("negative");
    }
    else {
        print!("neither positive nor negative");
    }
}
