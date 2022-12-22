use std::ops::{Add, Sub};

use num::{One, Zero};

use crate::Number;

/// When A has 2 strategies and B has 2 strategies
///
/// Then the payoff matrix for A is `[[a1, a2], [a3, a4]]` and for B is `[[b1, b2], [b3, b4]]`.
///
/// This function returns the mixed strategies for A and B.
pub fn mixed_strategies_2_2(a: &[Number; 4], b: &[Number; 4]) -> ([Number; 2], [Number; 2])
{
    let [b1, b2, b3, b4] = b;
    let q1 = b4.clone() - b3.clone();
    let q2 = b1.clone() - b2.clone() + b3.clone() - b4.clone();
    let q = if q2.is_zero() {
        panic!("q2 is zero")
    } else {
        make_rest(q1, q2)
    };
    (player_a(a), q)
}

fn make_rest(a: Number, b: Number) -> [Number; 2] {
    let y = Number::one() - (a.clone() / b.clone());
    let x = a / b;
    [x, y]
}

fn player_a(a: &[Number; 4]) -> [Number; 2] {
    // (a4 - a2) / (a1 - a2 - a3 + a4)
    let [a1, a2, a3, a4] = a;
    let denominator = a1.sub(a2).sub(a3.clone()).add(a4);
    if denominator.is_zero() {
        //
        if a1 == a2 && a3 == a4 {
            return [Number::fraction(1, 2), Number::fraction(1, 2)];
        } else if a1 == a2 && a3 < a4 {
            return [Number::one(), Number::zero()];
        } else if a1 < a2 && a3 == a4 {
            return [Number::zero(), Number::one()];
        } else if a1.sub(a2) == a4.sub(a3) {
            return [Number::fraction(1, 2), Number::fraction(1, 2)];
        } else {
            panic!("Invalid payoff matrix: {:?}", a);
        }
    } else {
        let p1 = (a4.sub(a2)) / denominator.clone();
        let p2 = (a1.sub(a3)) / denominator.clone();
        return [p1, p2];
    }
}



#[test]
fn test() {
    let a = &[Number::from(3), Number::from(-1), Number::from(-1), Number::from(0)];
    let b = &[Number::from(2), Number::from(3), Number::from(1), Number::from(0)];
    let (x, y) = mixed_strategies_2_2(a, b);
    println!("{:?}", x);
    println!("{:?}", y);
}