# Simple Substring Trait

## Usage

```rust
use string_utility::Substring;

fn main() {
    let some_text = "42Hello, world!".to_string();

    let result = some_text.substring(2..);
    let expected = "Hello, world!";
    assert_eq!(result, expected);

    let result = some_text.substring(9..=13);
    let expected = "world";
    assert_eq!(result, expected);
}
```
