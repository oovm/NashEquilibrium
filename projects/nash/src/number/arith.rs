use super::*;

impl Zero for Number {
    fn zero() -> Self {
        Number::Fraction(Ratio::from_integer(0.into()))
    }

    fn is_zero(&self) -> bool {
        match self {
            Number::Fraction(v) => {
                v.is_zero()
            }
            Number::Decimal(v) => { v.is_zero() }
        }
    }
}

impl One for Number {
    fn one() -> Self {
        Number::Fraction(Ratio::from_integer(1.into()))
    }
}


impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::Fraction(lhs), Number::Fraction(rhs)) => {
                lhs == rhs
            }
            (Number::Decimal(lhs), Number::Decimal(rhs)) => {
                lhs == rhs
            }
            _ => {
                false
            }
        }
    }
}

impl Add<Self> for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Fraction(lhs), Number::Fraction(rhs)) => {
                Number::Fraction(lhs + rhs)
            }
            (Number::Decimal(a), b) | (b, Number::Decimal(a)) => {
                Number::Decimal(a + b.as_float())
            }
        }
    }
}


impl Mul<Self> for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}


impl Sub<Self> for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Div<Self> for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Rem<Self> for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

// impl NumOps for Number {}

impl Num for Number {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}