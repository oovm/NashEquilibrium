use std::ops::{Add, Div, Mul, Rem, Sub};
use num::{BigInt, Num, One, Zero};
use num::rational::Ratio;
use num::traits::NumOps;
use num::traits::real::Real;

use dashu::{Rational, Real};

pub enum Number {
    Fraction(Ratio<BigInt>),
    Decimal(f64),
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Zero for Number {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl Add<Self, Output=Self> for Number {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl One for Number {
    fn one() -> Self {
        todo!()
    }
}

impl Mul<Self, Output=Self> for Number {
    type Output = ();

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl NumOps for Number {}

impl Sub<Self, Output=Self> for Number {
    type Output = ();

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Div<Self, Output=Self> for Number {
    type Output = ();

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Rem<Self, Output=Self> for Number {
    type Output = ();

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Num for Number {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Number::Fraction(Ratio::from_integer(value.into()))
    }
}