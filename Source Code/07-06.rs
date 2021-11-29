/* ILLEGAL. The compiler prints the error message:
expected expression, found keyword `fn`
*/
fn main() {
    #[allow(dead_code)]
    enum Continent {
        Europe,
        Asia,
        Africa,
        America,
        Oceania,
    }
    let contin = Continent::Asia;
    match contin {
        Continent::Europe => print!("E"),
        Continent::Asia => print!("As"),
        Continent::Africa => fn aaa() {},
        Continent::America => print!("Am"),
        Continent::Oceania => print!("O"),
    }
}
