/* It prints:
Hello, [Hi]*/
fn main() {
    struct Text { characters: String }
    impl Text {
        fn from(text: &str) -> Text {
            Text { characters: text.to_string() }
        }
        fn draw(&self) {
            print!("{}", self.characters);
        }
    }
    let greeting = Text::from("Hello");
    greeting.draw();
    struct BoxedText {
        text: Text,
        first: char,
        last: char,
    }
    impl BoxedText {
        fn with_text_and_borders(
            text: &str, first: char, last: char)
            -> BoxedText
        {
            BoxedText {
                text: Text::from(text),
                first: first,
                last: last,
            }
        }
        fn draw(&self) {
            print!("{}", self.first);
            self.text.draw();
            print!("{}", self.last);
        }
    }
    let boxed_greeting =
        BoxedText::with_text_and_borders("Hi", '[', ']');
    print!(", ");
    boxed_greeting.draw();
}
