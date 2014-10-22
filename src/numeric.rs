use std::char::{is_whitespace, to_digit};
use std::num;

#[deriving(Clone, Show, Eq)]
pub struct Numeric {
    value: i64,
    len: uint,
    precision: uint
}

impl Numeric {
    pub fn from_str(s: &str, len: uint, precision: uint) -> Option<Numeric> {
        let mut s = s.trim_chars(is_whitespace);
        let mut value = 0i64;
        let mut negative = false;
        let mut fraction = false;
        if s[0..1] == "-" {
            negative = true;
            s = s[1..];
        }

        if s.contains_char('.') {
            while s.chars().last() == Some('0') {
                s = s[..s.len() - 1]
            }
        }

        let mut digits_seen = 0u;
        let mut digits_seen_fraction = 0u;
        for c in s.chars() {
            if let Some(n) = to_digit(c, 10) {
                value = value * 10 + n as i64;
                if fraction {
                    digits_seen_fraction += 1;
                } else {
                    digits_seen += 1;
                }
            } else if c == '.' {
                fraction = match fraction {
                    true => return None,
                    false => true
                };
            } else {
                return None;
            }
        }

        if negative {
            value *= -1;
        }

        if digits_seen > len - precision || digits_seen_fraction > precision {
            None
        } else {
            Some(Numeric {
                value: value * num::pow(10, precision - digits_seen_fraction),
                len: len,
                precision: precision,
            })
        }
    }
}

impl PartialEq for Numeric {
    fn eq(&self, other: &Numeric) -> bool {
        self.value == other.value &&
            self.len == other.len &&
            self.precision == other.precision
    }
}

#[cfg(test)]
mod test {
    use super::Numeric;

    #[test]
    fn test_from_str() {
        assert_eq!(Numeric::from_str("50", 2, 0), Some(Numeric {value: 50, len: 2, precision: 0}))
        assert_eq!(Numeric::from_str("-50", 2, 0), Some(Numeric {value: -50, len: 2, precision: 0}))
        assert_eq!(Numeric::from_str("50.25", 4, 2), Some(Numeric {value: 5025, len: 4, precision: 2}))
        assert_eq!(Numeric::from_str("-50.25", 4, 2), Some(Numeric {value: -5025, len: 4, precision: 2}))
        assert_eq!(Numeric::from_str("-50.250", 4, 2), Some(Numeric {value: -5025, len: 4, precision: 2}))
        assert_eq!(Numeric::from_str("-50.25", 5, 3), Some(Numeric {value: -50250, len: 5, precision: 3}))
        assert_eq!(Numeric::from_str("10.2.1", 4, 0), None)
        assert_eq!(Numeric::from_str("abc", 4, 0), None)
    }
}
