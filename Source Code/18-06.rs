/* ILLEGAL. The compiler prints the error message:
no method named `naming` found for struct `Person` in the current scope
*/
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
    print!("{}", person.naming());
}
