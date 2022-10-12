# Simple Substring Trait

<!-- TOC -->
* [Simple Substring Trait](#simple-substring-trait)
  * [Install](#install)
  * [Usage](#usage)
    * [Substring](#substring)
    * [Keep](#keep)
<!-- TOC -->

## Install

Add the following line to your Cargo.toml file (under `[dependencies]`):

```toml
string-utility = "0.2"
```

## Usage

### Import all Traits

```rust
use string_utility::prelude::*;
```

### Substring

```rust
use string_utility::prelude::*;

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

All functions: [trait def](https://docs.rs/string-utility/0.2.0/string_utility/trait.StringKeeperExt.html)

```rust
use string_utility::prelude::*;

fn main() {
  let some_text = "some start value, some not-so-start value".to_string();
  
  let result = some_text
          .keep("start".to_string())  // keep(pattern)
          // from the
          .end_of_string()
          // whether to keep the pattern "start"
          .excluding_pattern() 
          // keep everything before the pattern
          .before_pattern()
          // exec
          .to_string();
    let expected = "some start value, some not-so-";
    assert_eq!(result, expected);
}
```
