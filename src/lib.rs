#![doc = include_str ! ("./../README.md")]
#![forbid(unsafe_code)]

pub mod prelude {
    pub use crate::{
        StringKeeperExt,
        SubstringExt,
        SubstringKeeperExt
    };
}

pub trait SubstringExt {
    fn substring<R: std::ops::RangeBounds<usize>>(&self, range: R) -> String;
    fn try_substring<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<String>;
}

pub trait SubstringKeeperExt<T> {
    fn keep(self, pattern: T) -> StringKeeper<T>;
}

pub trait StringKeeperExt<T> {
    fn beginning_of_string(self) -> StringKeeper<T>;
    fn end_of_string(self) -> StringKeeper<T>;
    fn including_pattern(self) -> StringKeeper<T>;
    fn excluding_pattern(self) -> StringKeeper<T>;
    fn before_pattern(self) -> StringKeeper<T>;
    fn after_pattern(self) -> StringKeeper<T>;
}

pub enum KeeperPeriod {
    Start,
    End,
}

pub enum KeeperCutoff {
    After,
    Before,
}

pub enum KeeperClusivity {
    Including,
    Excluding,
}

pub struct StringKeeper<T> {
    to_parse: String,
    pattern: T,
    period: KeeperPeriod,
    clusivity: KeeperClusivity,
    cutoff: KeeperCutoff,
}

#[cfg(feature = "regex")]
impl SubstringKeeperExt<regex::Regex> for String {
    fn keep(self, pattern: regex::Regex) -> StringKeeper<regex::Regex> {
        StringKeeper {
            to_parse: self,
            period: KeeperPeriod::Start,
            cutoff: KeeperCutoff::After,
            clusivity: KeeperClusivity::Including,
            pattern,
        }
    }
}

impl SubstringKeeperExt<String> for String {
    fn keep(self, pattern: String) -> StringKeeper<String> {
        StringKeeper {
            to_parse: self,
            period: KeeperPeriod::Start,
            cutoff: KeeperCutoff::After,
            clusivity: KeeperClusivity::Including,
            pattern,
        }
    }
}

impl SubstringKeeperExt<char> for String {
    fn keep(self, pattern: char) -> StringKeeper<char> {
        StringKeeper {
            to_parse: self,
            period: KeeperPeriod::Start,
            cutoff: KeeperCutoff::After,
            clusivity: KeeperClusivity::Including,
            pattern,
        }
    }
}

impl StringKeeperExt<String> for StringKeeper<String> {
    fn beginning_of_string(mut self) -> StringKeeper<String> {
        self.period = KeeperPeriod::Start;
        self
    }

    fn end_of_string(mut self) -> StringKeeper<String> {
        self.period = KeeperPeriod::End;
        self
    }

    fn including_pattern(mut self) -> StringKeeper<String> {
        self.clusivity = KeeperClusivity::Including;
        self
    }

    fn excluding_pattern(mut self) -> StringKeeper<String> {
        self.clusivity = KeeperClusivity::Excluding;
        self
    }

    fn before_pattern(mut self) -> StringKeeper<String> {
        self.cutoff = KeeperCutoff::Before;
        self
    }

    fn after_pattern(mut self) -> StringKeeper<String> {
        self.cutoff = KeeperCutoff::After;
        self
    }
}

impl std::fmt::Display for StringKeeper<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let try_find = match self.period {
            KeeperPeriod::Start => self.to_parse.find(&self.pattern),
            KeeperPeriod::End => self.to_parse.rfind(&self.pattern),
        };

        let range = match try_find {
            None => usize::MIN..usize::MIN,
            Some(pos) => match self.clusivity {
                KeeperClusivity::Including => match self.cutoff {
                    KeeperCutoff::After => pos..usize::MAX,
                    KeeperCutoff::Before => {
                        let offset = pos + self.pattern.chars().count();
                        usize::MIN..offset
                    }
                },
                KeeperClusivity::Excluding => match self.cutoff {
                    KeeperCutoff::After => {
                        let offset = pos + self.pattern.chars().count();
                        offset..usize::MAX
                    }
                    KeeperCutoff::Before => usize::MIN..pos,
                },
            },
        };

        write!(f, "{}", self.to_parse.substring(range))
    }
}

impl StringKeeperExt<char> for StringKeeper<char> {
    fn beginning_of_string(mut self) -> Self {
        self.period = KeeperPeriod::Start;
        self
    }

    fn end_of_string(mut self) -> Self {
        self.period = KeeperPeriod::End;
        self
    }

    fn including_pattern(mut self) -> Self {
        self.clusivity = KeeperClusivity::Including;
        self
    }

    fn excluding_pattern(mut self) -> Self {
        self.clusivity = KeeperClusivity::Excluding;
        self
    }

    fn before_pattern(mut self) -> Self {
        self.cutoff = KeeperCutoff::Before;
        self
    }

    fn after_pattern(mut self) -> Self {
        self.cutoff = KeeperCutoff::After;
        self
    }

}

impl std::fmt::Display for StringKeeper<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let try_find = match self.period {
            KeeperPeriod::Start => self.to_parse.find(self.pattern),
            KeeperPeriod::End => self.to_parse.rfind(self.pattern),
        };

        let range = match try_find {
            None => usize::MIN..usize::MIN,
            Some(pos) => match self.clusivity {
                KeeperClusivity::Including => match self.cutoff {
                    KeeperCutoff::After => pos..usize::MAX,
                    KeeperCutoff::Before => usize::MIN..(pos).saturating_add(1),
                },
                KeeperClusivity::Excluding => match self.cutoff {
                    KeeperCutoff::After => (pos).saturating_add(1)..usize::MAX,
                    KeeperCutoff::Before => usize::MIN..pos,
                },
            },
        };

        write!(f, "{}", self.to_parse.substring(range))
    }
}

#[cfg(feature = "regex")]
impl StringKeeperExt<regex::Regex> for StringKeeper<regex::Regex> {
    fn beginning_of_string(mut self) -> StringKeeper<regex::Regex> {
        self.period = KeeperPeriod::Start;
        self
    }

    fn end_of_string(mut self) -> StringKeeper<regex::Regex> {
        self.period = KeeperPeriod::End;
        self
    }

    fn including_pattern(mut self) -> StringKeeper<regex::Regex> {
        self.clusivity = KeeperClusivity::Including;
        self
    }

    fn excluding_pattern(mut self) -> StringKeeper<regex::Regex> {
        self.clusivity = KeeperClusivity::Excluding;
        self
    }

    fn before_pattern(mut self) -> StringKeeper<regex::Regex> {
        self.cutoff = KeeperCutoff::Before;
        self
    }

    fn after_pattern(mut self) -> StringKeeper<regex::Regex> {
        self.cutoff = KeeperCutoff::After;
        self
    }
}

#[cfg(feature = "regex")]
impl std::fmt::Display for StringKeeper<regex::Regex> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cloned_to_parse = self.to_parse.clone();
        let to_parse = cloned_to_parse.as_str();
        let try_find = match self.period {
            KeeperPeriod::Start => {
                self.pattern.find(to_parse)
            },
            KeeperPeriod::End => {
                self.pattern.find_iter(to_parse).last()
            },
        };

        let range = match try_find {
            None => usize::MIN..usize::MIN,
            Some(pos) => match self.clusivity {
                KeeperClusivity::Including => match self.cutoff {
                    KeeperCutoff::After => pos.start()..usize::MAX,
                    KeeperCutoff::Before => {
                        let offset = pos.as_str().chars().last().map(char::len_utf8).unwrap_or(2);
                        let end = pos.end().saturating_sub(offset);
                        usize::MIN..end
                    },
                },
                KeeperClusivity::Excluding => match self.cutoff {
                    KeeperCutoff::After => {
                        let offset = pos.as_str().chars().count();
                        let start = pos.start() + offset;
                        start..usize::MAX
                    },
                    KeeperCutoff::Before => usize::MIN..pos.start(),
                },
            },
        };

        write!(f, "{}", self.to_parse.substring(range))
    }
}

impl SubstringExt for str {
    fn substring<R: std::ops::RangeBounds<usize>>(&self, range: R) -> String {
        self.try_substring(range).unwrap_or_else(|| "".to_string())
    }

    fn try_substring<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<String> {
        let start_idx = match range.start_bound() {
            std::collections::Bound::Included(v) => *v,
            std::collections::Bound::Excluded(v) => v.saturating_add(1),
            std::collections::Bound::Unbounded => usize::MIN,
        };

        let end_idx = match range.end_bound() {
            std::collections::Bound::Included(v) => v.saturating_add(1),
            std::collections::Bound::Excluded(v) => *v,
            std::collections::Bound::Unbounded => usize::MAX,
        };

        if end_idx <= start_idx {
            return Some("".to_string());
        }

        end_idx
            .checked_sub(start_idx)
            .map(|take_count| self.chars().skip(start_idx).take(take_count).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

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
        assert_eq!("ã".substring(..1), "a"); // As opposed to "ã".
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
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep("karøbα".to_string())
                .beginning_of_string() // default
                .after_pattern() // default
                .including_pattern() // default
                .to_string(),
            "karøbα it was"
        );
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep("karøbα".to_string())
                .to_string(),
            "karøbα it was"
        );
        assert_eq!(
            "karøbα"
                .to_string()
                .keep("kar".to_string())
                .after_pattern()
                .including_pattern()
                .to_string(),
            "karøbα"
        );
    }

    #[test]
    fn test_keep_after_exclude_string() {
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep("karøbα".to_string())
                .beginning_of_string()
                .after_pattern()
                .excluding_pattern()
                .to_string(),
            " it was"
        );
        assert_eq!(
            "karøbα"
                .to_string()
                .keep("kar".to_string())
                .after_pattern()
                .excluding_pattern()
                .to_string(),
            "øbα"
        );
    }

    #[test]
    fn test_keep_after_include_char() {
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep('k')
                .after_pattern()
                .including_pattern()
                .to_string(),
            "karøbα it was"
        );
        assert_eq!(
            "karøbα"
                .to_string()
                .keep('k')
                .after_pattern()
                .including_pattern()
                .to_string(),
            "karøbα"
        );
    }

    #[test]
    fn test_keep_after_exclude_char() {
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep('k')
                .after_pattern()
                .excluding_pattern()
                .to_string(),
            "arøbα it was"
        );
        assert_eq!(
            "karøbα"
                .to_string()
                .keep('k')
                .after_pattern()
                .excluding_pattern()
                .to_string(),
            "arøbα"
        );
    }

    #[test]
    fn test_keep_before_include_string() {
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep("øbα".to_string())
                .before_pattern()
                .including_pattern()
                .to_string(),
            "this is karøbα"
        );
        assert_eq!(
            "karøbα"
                .to_string()
                .keep("øbα".to_string())
                .before_pattern()
                .including_pattern()
                .to_string(),
            "karøbα"
        );
    }
    #[test]
    fn test_keep_before_include_char() {
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep('ø')
                .before_pattern()
                .including_pattern()
                .to_string(),
            "this is karø"
        );
        assert_eq!(
            "karøbα"
                .to_string()
                .keep('ø')
                .before_pattern()
                .including_pattern()
                .to_string(),
            "karø"
        );
    }

    #[test]
    fn test_keep_before_exclude_string() {
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep("øbα".to_string())
                .before_pattern()
                .excluding_pattern()
                .to_string(),
            "this is kar"
        );
        assert_eq!(
            "karøbα"
                .to_string()
                .keep("øbα".to_string())
                .before_pattern()
                .excluding_pattern()
                .to_string(),
            "kar"
        );
    }

    #[test]
    fn test_keep_before_exclude_char() {
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep('ø')
                .before_pattern()
                .excluding_pattern()
                .to_string(),
            "this is kar"
        );
        assert_eq!(
            "karøbα"
                .to_string()
                .keep('ø')
                .before_pattern()
                .excluding_pattern()
                .to_string(),
            "kar"
        );
    }
}

#[cfg(test)]
#[cfg(feature = "regex")]
mod regex_feature_tests {
    use super::prelude::*;
    use regex::Regex;

    #[test]
    fn test_keep_after_include_string() {
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep(Regex::new("karøbα").unwrap())
                .beginning_of_string() // default
                .after_pattern() // default
                .including_pattern() // default
                .to_string(),
            "karøbα it was"
        );
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep(Regex::new("karøbα").unwrap())
                .to_string(),
            "karøbα it was"
        );
        assert_eq!(
            "karøbα"
                .to_string()
                .keep(Regex::new("kar").unwrap())
                .after_pattern()
                .including_pattern()
                .to_string(),
            "karøbα"
        );
    }

    #[test]
    fn test_keep_after_exclude_string() {
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep(Regex::new("karøbα").unwrap())
                .beginning_of_string()
                .after_pattern()
                .excluding_pattern()
                .to_string(),
            " it was"
        );
        assert_eq!(
            "karøbα"
                .to_string()
                .keep(Regex::new("kar").unwrap())
                .after_pattern()
                .excluding_pattern()
                .to_string(),
            "øbα"
        );
    }

    #[test]
    fn test_keep_before_include_string() {
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep(Regex::new("øbα").unwrap())
                .before_pattern()
                .including_pattern()
                .to_string(),
            "this is karøbα"
        );
        assert_eq!(
            "karøbα"
                .to_string()
                .keep(Regex::new("øbα").unwrap())
                .before_pattern()
                .including_pattern()
                .to_string(),
            "karøbα"
        );
    }

    #[test]
    fn test_keep_before_exclude_string() {
        assert_eq!(
            "this is karøbα it was"
                .to_string()
                .keep(Regex::new("øbα").unwrap())
                .before_pattern()
                .excluding_pattern()
                .to_string(),
            "this is kar"
        );
        assert_eq!(
            "karøbα"
                .to_string()
                .keep(Regex::new("øbα").unwrap())
                .before_pattern()
                .excluding_pattern()
                .to_string(),
            "kar"
        );
    }
}