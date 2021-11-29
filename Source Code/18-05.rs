/* It prints:
John Doe*/
fn main() {
    struct Person {
        personal_names: String,
        family_names: String,
    }
    fn naming(p: Person) -> String {
        format!("{} {}",
            p.personal_names,
            p.family_names)
    }
    let person = Person {
        personal_names: "John".to_string(),
        family_names: "Doe".to_string(),
    };
    print!("{}", naming(person));
}
