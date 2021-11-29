/* It prints:
John Doe*/
fn main() {
    struct Person {
        personal_names: String,
        family_names: String,
    }
    impl Person {
        fn naming(self) -> String {
            format!("{} {}",
                self.personal_names,
                self.family_names)
        }
    }
    let person = Person {
        personal_names: "John".to_string(),
        family_names: "Doe".to_string(),
    };
    print!("{}", person.naming());
}
