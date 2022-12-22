use std::cmp::Ordering;
use std::num::ParseFloatError;

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

impl Eq for Number {}

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

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_float().partial_cmp(&other.as_float())
    }
}

impl Add<Self> for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        add(&self, &rhs)
    }
}

impl Add<&Self> for Number {
    type Output = Number;

    fn add(self, rhs: &Self) -> Self::Output {
        add(&self, rhs)
    }
}

fn add(lhs: &Number, rhs: &Number) -> Number {
    match (lhs, rhs) {
        (Number::Fraction(lhs), Number::Fraction(rhs)) => {
            Number::Fraction(lhs + rhs)
        }
        (Number::Decimal(a), b) | (b, Number::Decimal(a)) => {
            Number::Decimal(a + b.as_float())
        }
    }
}
impl Sub<Self> for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        sub(&self, &rhs)
    }
}

impl Sub<&Number> for &Number {
    type Output = Number;

    fn sub(self, rhs: &Number) -> Self::Output {
        sub(&self, rhs)
    }
}

fn sub (lhs: &Number, rhs: &Number) -> Number {
    match (lhs, rhs) {
        (Number::Fraction(lhs), Number::Fraction(rhs)) => {
            Number::Fraction(lhs - rhs)
        }
        (Number::Decimal(a), b) | (b, Number::Decimal(a)) => {
            Number::Decimal(a - b.as_float())
        }
    }
}

impl Mul<Self> for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Fraction(lhs), Number::Fraction(rhs)) => {
                Number::Fraction(lhs * rhs)
            }
            (Number::Decimal(a), b) | (b, Number::Decimal(a)) => {
                Number::Decimal(a * b.as_float())
            }
        }
    }
}


impl Div<Self> for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Fraction(lhs), Number::Fraction(rhs)) => {
                Number::Fraction(lhs / rhs)
            }
            (Number::Decimal(a), b) | (b, Number::Decimal(a)) => {
                Number::Decimal(a / b.as_float())
            }
        }
    }
}

impl Rem<Self> for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Fraction(lhs), Number::Fraction(rhs)) => {
                Number::Fraction(lhs % rhs)
            }
            (Number::Decimal(a), b) | (b, Number::Decimal(a)) => {
                Number::Decimal(a % b.as_float())
            }
        }
    }
}

// impl NumOps for Number {}

impl Num for Number {
    type FromStrRadixErr = ParseFloatError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        match BigInt::from_str_radix(str, radix) {
            Ok(v) => {
                Ok(Number::Fraction(Ratio::from_integer(v)))
            }
            Err(_) => {
                Ok(Number::Decimal(str.parse::<f64>()?))
            }
        }
    }
}