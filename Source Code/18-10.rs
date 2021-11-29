/* It prints:
[ ] [John Doe] [Jane Doe]*/
fn main() {
    struct Person {
        personal_names: String,
        family_names: String,
    }
    impl Person {
        fn new() -> Self {
            Self {
                personal_names: String::new(),
                family_names: String::new(),
            }
        }
        fn naming(&self) -> String {
            format!("{} {}",
                self.personal_names,
                self.family_names)
        }
    }
    impl Person {
        fn set_personal_names(&mut self, new_name: String) {
            self.personal_names = new_name;
        }
    }
    let mut person = Person::new();
    print!("[{}] ", person.naming());
    person.personal_names = "John".to_string();
    person.family_names = "Doe".to_string();
    print!("[{}] ", person.naming());
    person.set_personal_names("Jane".to_string());
    print!("[{}]", person.naming());
}
