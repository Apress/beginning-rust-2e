/* It prints:
As*/
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
        Continent::Africa => print!("Af"),
        Continent::America => print!("Am"),
        Continent::Oceania => print!("O"),
    }
}
