use num::{One, Zero};

use crate::Number;

pub fn mixed_strategies2x2<L, R>(lhs: &[L; 4], rhs: &[R; 4]) -> ([Number; 2], [Number; 2])
    where L: Into<Number>
    , R: Into<Number>
{
    let lhs = (lhs[0].into(), lhs[1].into(), lhs[2].into(), lhs[3].into());
    let rhs = (rhs[0].into(), rhs[1].into(), rhs[2].into(), rhs[3].into());
    // p = (a4 - a2) /  (a1 - a2 - a3 + a4)
    // q = (b4 - b3) /  (b1 - b2 + b3 - b4)
    let p1 = lhs.3.clone() - lhs.1.clone();
    let p2 = lhs.0 - lhs.1 - lhs.2 + lhs.3;
    let q1 = rhs.3.clone() - rhs.2.clone();
    let q2 = rhs.0 - rhs.1 + rhs.2 - rhs.3;
    if p2.is_zero() {
        panic!("p2 is zero");
    }
    if q2.is_zero() {
        panic!("q2 is zero");
    }
    (make_rest(p1, p2), make_rest(q1, q2))
}

fn make_rest(a: Number, b: Number) -> [Number; 2] {
    let y = Number::one() - (a.clone() / b.clone());
    let x = a / b;
    [x, y]
}