use super::*;

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Fraction(v) => {
                if v.denom().is_one() {
                    Debug::fmt(v.numer(), f)
                } else {
                    write!(f, "{}/{}", v.numer(), v.denom())
                }
            }
            Number::Decimal(v) => {
                Debug::fmt(v, f)
            }
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<isize> for Number {
    fn from(value: isize) -> Self {
        Number::Fraction(Ratio::from_integer(value.into()))
    }
}