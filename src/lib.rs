#![doc = include_str ! ("./../README.md")]
#![forbid(unsafe_code)]

pub mod prelude {
    pub use crate::{
        SubstringExt,
        StringKeeperCommonExt,
        KeeperCommonExt,
    };
}

pub trait SubstringExt {
    fn substring<R: std::ops::RangeBounds<usize>>(&self, range: R) -> String;
    fn substring_len(&self, reverse_count: usize) -> String;
    fn try_substring<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<String>;
    fn trim_trailing_zeros(&self) -> String;
}

pub trait StringKeeperCommonExt<T, P> {
    fn keep(self, pattern: T) -> StringKeeper<T, P>;
    fn cut(self, pattern: T) -> StringKeeper<T, P>;
}

pub trait KeeperCommonExt<T, P> {
    fn beginning_of_string(self) -> StringKeeper<T, P>;
    fn end_of_string(self) -> StringKeeper<T, P>;
    fn including_pattern(self) -> StringKeeper<T, P>;
    fn excluding_pattern(self) -> StringKeeper<T, P>;
    fn before_pattern(self) -> StringKeeper<T, P>;
    fn after_pattern(self) -> StringKeeper<T, P>;
    fn until_first_matched_pattern(self, until_pattern: T) -> StringKeeper<T, P>;
    fn until_no_matched_pattern(self, until_pattern: T) -> StringKeeper<T, P>;

    #[cfg(feature = "regex")]
    fn utf8_encoding(self) -> StringKeeper<T, P>;

    #[cfg(feature = "regex")]
    fn utf16_encoding(self) -> StringKeeper<T, P>;

    #[cfg(feature = "regex")]
    fn set_encoding(self, enc: KeeperEncoding) -> StringKeeper<T, P>;
}

pub trait StringKeeperExt<T, P>: StringKeeperCommonExt<T, P> {}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum KeeperPeriod {
    Start,
    End,
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum KeeperCutoff {
    After,
    Before,
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum KeeperClusivity {
    Including,
    Excluding,
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum KeeperEncoding {
    Utf8,
    Utf16,
    // Other(fn(original_text: &str, matched_text: &str, last_char: char) -> usize),
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum KeeperUntilMatch {
    FirstMatch,
    NoMatch,
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum StringKeeperMode {
    Cut,
    Keep,
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct StringKeeperOpts {
    until_match: Option<KeeperUntilMatch>,
    mode: StringKeeperMode,
    period: KeeperPeriod,
    clusivity: KeeperClusivity,
    cutoff: KeeperCutoff,
    encoding: Option<KeeperEncoding>,
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct StringKeeper<T, P> {
    to_parse: P,
    pattern: T,
    until_pattern: Option<T>,
    opt: StringKeeperOpts,
}

impl<T, P> StringKeeperCommonExt<T, P> for P {
    fn keep(self, pattern: T) -> StringKeeper<T, P> {
        StringKeeper {
            pattern,
            to_parse: self,
            until_pattern: None,
            opt: StringKeeperOpts {
                until_match: None,
                mode: StringKeeperMode::Keep,
                period: KeeperPeriod::Start,
                cutoff: KeeperCutoff::After,
                clusivity: KeeperClusivity::Including,
                encoding: None,
            },
        }
    }

    fn cut(self, pattern: T) -> StringKeeper<T, P> {
        StringKeeper {
            pattern,
            to_parse: self,
            until_pattern: None,
            opt: StringKeeperOpts {
                until_match: None,
                mode: StringKeeperMode::Cut,
                period: KeeperPeriod::Start,
                cutoff: KeeperCutoff::After,
                clusivity: KeeperClusivity::Including,
                encoding: None,
            },
        }
    }
}

impl<T, P> KeeperCommonExt<T, P> for StringKeeper<T, P> {
    fn beginning_of_string(mut self) -> StringKeeper<T, P> {
        self.opt.period = KeeperPeriod::Start;
        self
    }

    fn end_of_string(mut self) -> StringKeeper<T, P> {
        self.opt.period = KeeperPeriod::End;
        self
    }

    fn including_pattern(mut self) -> StringKeeper<T, P> {
        self.opt.clusivity = KeeperClusivity::Including;
        self
    }

    fn excluding_pattern(mut self) -> StringKeeper<T, P> {
        self.opt.clusivity = KeeperClusivity::Excluding;
        self
    }

    fn before_pattern(mut self) -> StringKeeper<T, P> {
        self.opt.cutoff = KeeperCutoff::Before;
        self
    }

    fn after_pattern(mut self) -> StringKeeper<T, P> {
        self.opt.cutoff = KeeperCutoff::After;
        self
    }

    fn until_first_matched_pattern(mut self, until_pattern: T) -> StringKeeper<T, P> {
        self.until_pattern = Some(until_pattern);
        self.opt.until_match = Some(KeeperUntilMatch::FirstMatch);
        self
    }

    fn until_no_matched_pattern(mut self, until_pattern: T) -> StringKeeper<T, P> {
        self.until_pattern = Some(until_pattern);
        self.opt.until_match = Some(KeeperUntilMatch::NoMatch);
        self
    }

    #[cfg(feature = "regex")]
    fn utf8_encoding(mut self) -> StringKeeper<T, P> {
        self.opt.encoding = Some(KeeperEncoding::Utf8);
        self
    }

    #[cfg(feature = "regex")]
    fn utf16_encoding(mut self) -> StringKeeper<T, P> {
        self.opt.encoding = Some(KeeperEncoding::Utf16);
        self
    }

    #[cfg(feature = "regex")]
    fn set_encoding(mut self, enc: KeeperEncoding) -> StringKeeper<T, P> {
        self.opt.encoding = Some(enc);
        self
    }
}

impl std::fmt::Display for StringKeeper<String, String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let try_find = match self.opt.period {
            KeeperPeriod::Start => self.to_parse.find(&self.pattern),
            KeeperPeriod::End => self.to_parse.rfind(&self.pattern),
        };

        let range = match try_find {
            None => usize::MIN..usize::MIN,
            Some(pos) => match self.opt.clusivity {
                KeeperClusivity::Including => match self.opt.cutoff {
                    KeeperCutoff::After => pos..usize::MAX,
                    KeeperCutoff::Before => {
                        let offset = pos + self.pattern.chars().count();
                        usize::MIN..offset
                    }
                },
                KeeperClusivity::Excluding => match self.opt.cutoff {
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

impl std::fmt::Display for StringKeeper<char, String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let try_find = match self.opt.period {
            KeeperPeriod::Start => self.to_parse.find(self.pattern),
            KeeperPeriod::End => self.to_parse.rfind(self.pattern),
        };

        let range = match try_find {
            None => usize::MIN..usize::MIN,
            Some(pos) => match self.opt.clusivity {
                KeeperClusivity::Including => match self.opt.cutoff {
                    KeeperCutoff::After => pos..usize::MAX,
                    KeeperCutoff::Before => usize::MIN..(pos).saturating_add(1),
                },
                KeeperClusivity::Excluding => match self.opt.cutoff {
                    KeeperCutoff::After => (pos).saturating_add(1)..usize::MAX,
                    KeeperCutoff::Before => usize::MIN..pos,
                },
            },
        };

        let mut result = self.to_parse.substring(range);

        let opt_range = if let Some(until_pattern) = self.until_pattern {
            if let Some(until_match) = self.opt.until_match.clone() {
                let try_find = match self.opt.period {
                    KeeperPeriod::Start => result.find(until_pattern),
                    KeeperPeriod::End => result.rfind(until_pattern),
                };

                if let Some(pos) = try_find {
                    match until_match {
                        KeeperUntilMatch::FirstMatch => {
                            match self.opt.cutoff {
                                KeeperCutoff::After => {
                                    result
                                        .substring(pos..)
                                        .find(until_pattern)
                                        .map(|start_idx| start_idx..usize::MAX)
                                }
                                KeeperCutoff::Before => {
                                    result
                                        .substring(..=pos)
                                        .rfind(until_pattern)
                                        .map(|end_idx| pos..end_idx)
                                }
                            }
                        }
                        KeeperUntilMatch::NoMatch => {
                            match self.opt.cutoff {
                                KeeperCutoff::After => {
                                    let found = {
                                        let mut flag = false;
                                        result
                                            .chars()
                                            .enumerate()
                                            .skip(pos)
                                            .find(|(c_idx, c)| {
                                                if *c_idx > pos {
                                                    let same = *c == until_pattern;
                                                    if flag && !same {
                                                        return true;
                                                    }

                                                    if same {
                                                        flag = true;
                                                    }
                                                }

                                                false
                                            })
                                    };

                                    if let Some((start_pos, _)) = found {
                                        Some(start_pos.saturating_add(1)..pos.saturating_add(1))
                                    } else {
                                        None
                                    }
                                }
                                KeeperCutoff::Before => {
                                    let col = result
                                        .chars()
                                        .enumerate()
                                        .collect::<Vec<(usize, char)>>();

                                    let found = {
                                        let mut flag = false;
                                        col
                                            .iter()
                                            .rev()
                                            .find(|(c_idx, c)| {
                                                if *c_idx <= pos {
                                                    let same = *c == until_pattern;
                                                    if flag && !same {
                                                        return true;
                                                    }

                                                    if same {
                                                        flag = true;
                                                    }
                                                }

                                                false
                                            })
                                    };

                                    if let Some(&(start_pos, _)) = found {
                                        Some(start_pos.saturating_add(1)..pos.saturating_add(1))
                                    } else {
                                        None
                                    }
                                }
                            }
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let result = if let Some(range) = opt_range {
            match self.opt.mode {
                StringKeeperMode::Cut => {
                    result.replace_range(range, "");
                    result
                }
                StringKeeperMode::Keep => {
                    result.substring(range)
                }
            }
        } else {
            result
        };
        write!(f, "{}", result)
    }
}

#[cfg(feature = "regex")]
impl std::fmt::Display for StringKeeper<regex::Regex, String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_parse = self.to_parse.as_str();
        let try_find = match self.opt.period {
            KeeperPeriod::Start => {
                self.pattern.find(to_parse)
            }
            KeeperPeriod::End => {
                self.pattern.find_iter(to_parse).last()
            }
        };

        let range = match try_find {
            None => usize::MIN..usize::MIN,
            Some(pos) => match self.opt.clusivity {
                KeeperClusivity::Including => match self.opt.cutoff {
                    KeeperCutoff::After => pos.start()..usize::MAX,
                    KeeperCutoff::Before => {
                        let end_offset = if let Some(enc) = self.opt.encoding.clone() {
                            let matched_string = pos.as_str();
                            let char_len = matched_string
                                .chars()
                                .last()
                                .map(|last_char| {
                                    match enc {
                                        KeeperEncoding::Utf8 => char::len_utf8(last_char),
                                        KeeperEncoding::Utf16 => char::len_utf16(last_char),
                                    }
                                })
                                .unwrap_or(std::mem::size_of::<char>());
                            pos.end().saturating_sub(char_len)
                        } else {
                            pos.end()
                        };
                        usize::MIN..end_offset
                    }
                },
                KeeperClusivity::Excluding => match self.opt.cutoff {
                    KeeperCutoff::After => {
                        let offset = pos.as_str().chars().count();
                        let start = pos.start() + offset;
                        start..usize::MAX
                    }
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

    fn substring_len(&self, reverse_count: usize) -> String {
        self.substring(self.len().saturating_sub(reverse_count)..)
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

        if end_idx > start_idx {
            end_idx
                .checked_sub(start_idx)
                .map(|take_count| {
                    self
                        .chars()
                        .skip(start_idx)
                        .take(take_count)
                        .collect()
                })
        } else {
            None
        }
    }

    fn trim_trailing_zeros(&self) -> String {
        self
            .to_string()
            .cut('0')
            .end_of_string()
            .until_no_matched_pattern('0')
            .before_pattern()
            .excluding_pattern()
            .to_string()
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
        assert_eq!(any_string.substring_len(4), "illa");
        assert_eq!(any_string.substring_len(5), "zilla");
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

        assert_eq!(
            "3.141592650991234200000000000000000000"
                .to_string()
                .keep('0')
                .until_no_matched_pattern('0')
                .beginning_of_string()
                .before_pattern()
                .excluding_pattern()
                .to_string(),
            "3.14159265"
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
                .utf8_encoding()
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

        assert_eq!(
            "My numbør is 555-0100 and this is some other useless information"
                .to_string()
                .keep(Regex::new(r"\d{3}-\d{4}").unwrap())
                .utf8_encoding()
                .before_pattern()
                .including_pattern()
                .to_string(),
            "My numbør is 555-0100"
        );

        assert_eq!(
            "My number is 555-0100 and this is some other useless information"
                .to_string()
                .keep(Regex::new(r"\d{3}-\d{4}").unwrap())
                .before_pattern()
                .including_pattern()
                .to_string(),
            "My number is 555-0100"
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

#[cfg(test)]
mod pattern_keep_until {
    use crate::prelude::*;

    #[test]
    fn to_keep_until_pattern() {
        assert_eq!(
            "42.141592650991234200000000000000000000"
                .to_string()
                .cut('0')
                .end_of_string()
                .until_no_matched_pattern('0')
                .before_pattern()
                .excluding_pattern()
                .to_string(),
            "42.1415926509912342"
        );

        assert_eq!(
            "42.141592650991234200000000000000000000".trim_trailing_zeros(),
            "42.1415926509912342"
        );

    }
}