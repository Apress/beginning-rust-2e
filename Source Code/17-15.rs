/* It prints:
5
Error: "negative argument"
*/
fn main() -> Result<(), String> {
    fn incremented(n: i32) -> Result<i32, String> {
        if n < 0 { Err(format!("negative argument")) }
        else { Ok(n + 1) }
    }
    println!("{} ", incremented(4)?);
    println!("{} ", incremented(-3)?);
    println!("{} ", incremented(7)?);
    Ok(())
}
