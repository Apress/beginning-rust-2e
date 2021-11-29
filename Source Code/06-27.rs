/* It prints:
true false A à*/
fn main() {
    let truthy = 1;
    let falsy = 0;
    print!("{} {} {} {}", truthy != 0, falsy != 0,
        65 as char, 224 as char);    
}
