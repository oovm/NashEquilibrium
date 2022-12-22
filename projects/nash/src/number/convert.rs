
use super::*;



impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}



impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Number::Fraction(Ratio::from_integer(value.into()))
    }
}