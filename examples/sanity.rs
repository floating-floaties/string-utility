use string_utility::prelude::*;


fn main() {
    let some_text = "42Hello, world!".to_string();
    let result = some_text.substring(2..);
    let expected = "Hello, world!";
    assert_eq!(result, expected);

    let some_text = "42Hello, world!".to_string();
    let result = some_text.keep(",".to_string())
        .after_pattern()
        .excluding_pattern()
        .to_string();
    let expected = " world!";
    assert_eq!(result, expected);

    let some_text = "42Hello, world!".to_string();
    let result = some_text.keep(',')
        .after_pattern()
        .excluding_pattern()
        .to_string();

    let expected = " world!";
    assert_eq!(result, expected);
}