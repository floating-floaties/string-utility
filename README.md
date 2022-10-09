# Simple Substring Trait

<!-- TOC -->
* [Simple Substring Trait](#simple-substring-trait)
  * [Usage](#usage)
    * [Substring](#substring)
    * [Keep](#keep)
      * [Keep after (including)](#keep-after--including-)
      * [Keep after (excluding)](#keep-after--excluding-)
      * [Keep before (including)](#keep-before--including-)
      * [Keep before (excluding)](#keep-before--excluding-)
<!-- TOC -->

## Usage

### Substring

`String::substring(range: Range<usize>)`

```rust
use string_utility::Substring;

fn main() {
    let some_text = "42Hello, world!".to_string();

    // cut first two chars
    let result = some_text.substring(2..);
    let expected = "Hello, world!";
    assert_eq!(result, expected);

    // extract world string
    let result = some_text.substring(9..=13);
    let expected = "world";
    assert_eq!(result, expected);

    // keep everything up to (including) idx 13
    let result = some_text.substring(..=13);
    let expected = "42Hello, world";
    assert_eq!(result, expected);

    // keep everything up to (excluding) idx 13
    let result = some_text.substring(..13);
    let expected = "42Hello, worl";
    assert_eq!(result, expected);

    // keep everything
    let result = some_text.substring(..);
    let expected = "42Hello, world!";
    assert_eq!(result, expected);

    // get last char
    let result = some_text.substring(some_text.len()-1..);
    let expected = "!";
    assert_eq!(result, expected);
}
```


### Keep

#### Keep after (including)

```rust
use string_utility::{Substring, SubstringKeep};

fn main() {
  let some_text = "Password: mYsuperSecretPassword -- so secretive [gasp]".to_string();

  // with string
  let result = some_text.keep_after_include("-- ");
  let expected = "-- so secretive [gasp]";
  assert_eq!(result, expected);

  // with chars
  let result = some_text.keep_after_include('-');
  let expected = "-- so secretive [gasp]";
  assert_eq!(result, expected);
}
```

#### Keep after (excluding)

```rust
use string_utility::{Substring, SubstringKeep};


fn main() {
  let some_text = "Password: mYsuperSecretPassword -- so secretive [gasp]".to_string();

  // with string
  let result = some_text.keep_after_exclude("-- ");
  let expected = "so secretive [gasp]";
  assert_eq!(result, expected);

  // with chars
  let result = some_text.keep_after_exclude('-');
  let expected = "- so secretive [gasp]";
  assert_eq!(result, expected);
}
```

#### Keep before (including)

```rust
use string_utility::{Substring, SubstringKeep};

fn main() {
  let some_text = "Password: mYsuperSecretPassword -- so secretive [gasp]".to_string();

  // with string
  let result = some_text.keep_before_include("-- ");
  let expected = "Password: mYsuperSecretPassword -- ";
  assert_eq!(result, expected);

  // with chars
  let result = some_text.keep_before_include('-');
  let expected = "Password: mYsuperSecretPassword -";
  assert_eq!(result, expected);
}
```

#### Keep before (excluding)

```rust
use string_utility::{Substring, SubstringKeep};


fn main() {
  let some_text = "Password: mYsuperSecretPassword -- so secretive [gasp]".to_string();

  // with string
  let result = some_text.keep_before_exclude("-- ");
  let expected = "Password: mYsuperSecretPassword ";
  assert_eq!(result, expected);

  // with chars
  let result = some_text.keep_before_exclude('-');
  assert_eq!(result, expected);
}
```