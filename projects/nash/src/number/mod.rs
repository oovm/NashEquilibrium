use std::fmt::{Debug, Formatter};
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Rem, Sub};

use num::{BigInt, Num, One, ToPrimitive, Zero};
use num::rational::Ratio;

mod arith;
mod convert;

#[derive(Clone)]
pub enum Number {
    Fraction(Ratio<BigInt>),
    Decimal(f64),
}

impl Number {
    pub fn fraction<A, B>(n: A, d: B) -> Self
        where A: Into<BigInt>,
              B: Into<BigInt>
    {
        Number::Fraction(Ratio::new(n.into(), d.into()))
    }


    pub fn as_float(&self) -> f64 {
        match self {
            Number::Fraction(r) => {
                r.to_f64().unwrap_or(0.0)
            }
            Number::Decimal(f) => { *f }
        }
    }
    pub fn as_ratio(&self) -> Ratio<BigInt> {
        match self {
            Number::Fraction(r) => { r.clone() }
            Number::Decimal(f) => {
                panic!("Cannot convert decimal `{}` to ratio", f)
            }
        }
    }
}