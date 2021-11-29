/* It prints:
48 35 20 39 42 33 50 29 27 18 17 16 13 12 */
fn main() {
    let data = vec![
        48, 18, 20, 35, 17, 13, 39, 12, 42, 33, 29, 27, 50, 16];
    let mut v = Vec::<i32>::new();
    fn add(v: &mut Vec<i32>, item: i32) {
        v.push(item);
        v.sort();
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
