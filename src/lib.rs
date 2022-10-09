#![doc = include_str ! ("./../README.md")]
#![forbid(unsafe_code)]

use std::collections::Bound;
use std::ops::RangeBounds;

pub trait Substring {
    fn substring<R: RangeBounds<usize>>(&self, range: R) -> String;
    fn try_substring<R: RangeBounds<usize>>(&self, range: R) -> Option<String>;
}

pub trait SubstringKeep<S> {
    fn keep_after_include(&self, pattern: S) -> String;
    fn keep_after_exclude(&self, pattern: S) -> String;
    fn keep_before_include(&self, pattern: S) -> String;
    fn keep_before_exclude(&self, pattern: S) -> String;
}


impl SubstringKeep<&str> for str {
    fn keep_after_include(&self, pattern: &str) -> String {
        match self.find(pattern) {
            None => "".to_string(),
            Some(pos) => self.substring(pos..)
        }
    }


    fn keep_after_exclude(&self, pattern: &str) -> String {
        match self.find(pattern) {
            None => "".to_string(),
            Some(pos) => self.substring(pos + pattern.chars().count()..)
        }
    }

    fn keep_before_include(&self, pattern: &str) -> String {
        match self.find(pattern) {
            None => "".to_string(),
            Some(pos) => self.substring(..pos + pattern.chars().count())
        }
    }

    fn keep_before_exclude(&self, pattern: &str) -> String {
        match self.find(pattern) {
            None => "".to_string(),
            Some(pos) => self.substring(..pos)
        }
    }
}

impl SubstringKeep<char> for str {
    fn keep_after_include(&self, pattern: char) -> String {
        match self.find(pattern) {
            None => "".to_string(),
            Some(pos) => self.substring(pos..)
        }
    }


    fn keep_after_exclude(&self, pattern: char) -> String {
        match self.find(pattern) {
            None => "".to_string(),
            Some(pos) => self.substring(pos.saturating_add(1)..)
        }
    }

    fn keep_before_include(&self, pattern: char) -> String {
        match self.find(pattern) {
            None => "".to_string(),
            Some(pos) => self.substring(..=pos)
        }
    }

    fn keep_before_exclude(&self, pattern: char) -> String {
        match self.find(pattern) {
            None => "".to_string(),
            Some(pos) => self.substring(..pos)
        }
    }
}


impl Substring for str {
    fn substring<R: RangeBounds<usize>>(&self, range: R) -> String {
        self.try_substring(range).unwrap_or_else(|| "".to_string())
    }

    fn try_substring<R: RangeBounds<usize>>(&self, range: R) -> Option<String> {
        let start_idx = match range.start_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => v.saturating_add(1),
            Bound::Unbounded => usize::MIN,
        };

        let end_idx = match range.end_bound() {
            Bound::Included(v) => v.saturating_add(1),
            Bound::Excluded(v) => *v,
            Bound::Unbounded => usize::MAX,
        };

        if end_idx <= start_idx {
            return Some("".to_string());
        }

        end_idx
            .checked_sub(start_idx)
            .map(|take_count| {
                self
                    .chars()
                    .skip(start_idx)
                    .take(take_count)
                    .collect()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_substring() {
        let some_text = "hello, world!";
        let result = some_text.substring(7..12);
        assert_eq!(result, "world");

        let some_text = "42Hello, world!".to_string();
        let result = some_text.try_substring(2..7).unwrap();
        let expected = "Hello";
        assert_eq!(result, expected);
    }

    #[test]
    fn substring() {
        let some_text = "42Hello, world!".to_string();

        let result = some_text.substring(2..7);
        let expected = "Hello";
        assert_eq!(result, expected);


        let result = some_text.substring(2..424242);
        let expected = "Hello, world!";
        assert_eq!(result, expected);

    }

    #[test]
    fn test_substring() {
        assert_eq!("foobar".substring(..3), "foo");
    }

    #[test]
    fn test_out_of_bounds() {
        assert_eq!("foobar".substring(..10), "foobar");
        assert_eq!("foobar".substring(6..10), "");
    }

    #[test]
    fn test_start_and_end_equal() {
        assert_eq!("foobar".substring(3..3), "");
    }

    #[test]
    fn test_multiple_byte_characters() {
        assert_eq!("ã".substring(..1), "a");  // As opposed to "ã".
        assert_eq!("ã".substring(1..2), "\u{0303}");
        assert_eq!("fõøbα®".substring(2..5), "øbα");
    }

    #[test]
    fn mozilla_substring_cases() {
        let any_string = "Mozilla";
        assert_eq!(any_string.substring(..1), "M");
        assert_eq!(any_string.substring(1..), "ozilla");
        assert_eq!(any_string.substring(..6), "Mozill");
        assert_eq!(any_string.substring(4..), "lla");
        assert_eq!(any_string.substring(4..7), "lla");
        assert_eq!(any_string.substring(..7), "Mozilla");
        assert_eq!(any_string.substring(..10), "Mozilla");
        assert_eq!(any_string.substring(any_string.len() - 4..), "illa");
        assert_eq!(any_string.substring(any_string.len() - 5..), "zilla");
        assert_eq!(any_string.substring(2..5), "zil");
        assert_eq!(any_string.substring(..2), "Mo");
        assert_eq!(any_string.substring(..), "Mozilla");
    }

    #[test]
    fn test_keep_after_include_string() {
        assert_eq!("this is karøbα it was".keep_after_include("karøbα"), "karøbα it was");
        assert_eq!("karøbα".keep_after_include("kar"), "karøbα");
    }

    #[test]
    fn test_keep_after_exclude_string() {
        assert_eq!("this is karøbα it was".keep_after_exclude("karøbα"), " it was");
        assert_eq!("karøbα".keep_after_exclude("kar"), "øbα");
    }

    #[test]
    fn test_keep_after_include_char() {
        assert_eq!("this is karøbα it was".keep_after_include('k'), "karøbα it was");
        assert_eq!("karøbα".keep_after_include('k'), "karøbα");
    }

    #[test]
    fn test_keep_after_exclude_char() {
        assert_eq!("this is karøbα it was".keep_after_exclude('k'), "arøbα it was");
        assert_eq!("karøbα".keep_after_exclude('k'), "arøbα");
    }

    #[test]
    fn test_keep_before_include_string() {
        assert_eq!("this is karøbα it was".keep_before_include("øbα"), "this is karøbα");
        assert_eq!("karøbα".keep_before_include("øbα"), "karøbα");
    }
    #[test]
    fn test_keep_before_include_char() {
        assert_eq!("this is karøbα it was".keep_before_include('ø'), "this is karø");
        assert_eq!("karøbα".keep_before_include('ø'), "karø");
    }


    #[test]
    fn test_keep_before_exclude_string() {
        assert_eq!("this is karøbα it was".keep_before_exclude("øbα"), "this is kar");
        assert_eq!("karøbα".keep_before_exclude("øbα"), "kar");
    }

    #[test]
    fn test_keep_before_exclude_char() {
        assert_eq!("this is karøbα it was".keep_before_exclude('ø'), "this is kar");
        assert_eq!("karøbα".keep_before_exclude('ø'), "kar");
    }
}
