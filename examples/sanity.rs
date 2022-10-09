use string_utility::{Substring};

fn main() {
    let some_text = "42Hello, world!".to_string();

    let result = some_text.substring(2..);
    let expected = "Hello";
    assert_eq!(result, expected);
}