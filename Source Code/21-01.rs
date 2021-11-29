/* It prints all the integer numbers from 0 to 9999,
and then the time spent for such prints:
*/
fn main() {
    let start_time = std::time::Instant::now();
    for i in 0..10_000 {
        println!("{}", i);
    }
    println!("{:?}", start_time.elapsed());
}
