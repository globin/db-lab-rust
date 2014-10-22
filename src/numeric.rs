use std::char::{is_whitespace, to_digit};
use std::cmp::max;
use std::num::pow;

#[deriving(PartialOrd, Clone, Show, Eq)]
pub struct Numeric {
    value: i64,
    len: uint,
    precision: uint
}

impl Numeric {
    pub fn new(value: i64, len: uint, precision: uint) -> Numeric {
        // TODO consistency check
        Numeric {
            value: value,
            len: len,
            precision: precision
        }
    }

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
            Some(Numeric::new(value * pow(10, precision - digits_seen_fraction), len, precision))
        }
    }
}

impl PartialEq for Numeric {
    fn eq(&self, other: &Numeric) -> bool {
        self.value == other.value
            && self.precision == other.precision
    }
}

impl Ord for Numeric {
    fn cmp(&self, other: &Numeric) -> Ordering {
        match self.precision.cmp(&other.precision) {
            Equal => self.value.cmp(&other.value),
            Less => (self.value * pow(10, other.precision - self.precision)).cmp(&other.value),
            Greater => (other.value * pow(10, self.precision - other.precision)).cmp(&self.value),
        }
    }
}

impl Add<Numeric, Numeric> for Numeric {
    fn add(&self, rhs: &Numeric) -> Numeric {
        Numeric {
            value: match self.precision.cmp(&rhs.precision) {
                Equal => self.value + rhs.value,
                Less => self.value * pow(10, rhs.precision - self.precision) + rhs.value,
                Greater => rhs.value * pow(10, self.precision - rhs.precision) + self.value,
            },
            precision: max(self.precision, rhs.precision),
            len: max(self.len, rhs.len)
        }
    }
}

impl Sub<Numeric, Numeric> for Numeric {
    fn sub(&self, rhs: &Numeric) -> Numeric {
        Numeric {
            value: match self.precision.cmp(&rhs.precision) {
                Equal => self.value - rhs.value,
                Less => self.value * pow(10, rhs.precision - self.precision) - rhs.value,
                Greater => self.value - rhs.value * pow(10, self.precision - rhs.precision),
            },
            precision: max(self.precision, rhs.precision),
            len: max(self.len, rhs.len)
        }
    }
}

impl Mul<Numeric, Numeric> for Numeric {
    fn mul(&self, rhs: &Numeric) -> Numeric {
        Numeric {
            value: match self.precision.cmp(&rhs.precision) {
                Equal => self.value * rhs.value,
                Less => self.value * pow(10, rhs.precision - self.precision) * rhs.value,
                Greater => self.value * rhs.value * pow(10, self.precision * rhs.precision),
            },
            precision: max(self.precision, rhs.precision),
            len: max(self.len, rhs.len)
        }
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
