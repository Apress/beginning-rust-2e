/* It prints:
18 35 13 12 33 27 16 50 29 42 39 17 20 48 */
fn main() {
    let data = vec![
        48, 18, 20, 35, 17, 13, 39, 12, 42, 33, 29, 27, 50, 16];
    let mut v = Vec::<i32>::new();
    fn add(v: &mut Vec<i32>, item: i32) {
        v.push(item);
    }
    fn extract(v: &mut Vec<i32>) -> i32 {
        v.pop().unwrap()
    }
    let mut i = 0;
    loop {
        if i == data.len() { break; }
        add(&mut v, data[i]);
        i += 1;
        if i == data.len() { break; }
        add(&mut v, data[i]);
        i += 1;
        print!("{} ", extract(&mut v));
    }
    while ! v.is_empty() {
        print!("{} ", extract(&mut v));
    }
}
