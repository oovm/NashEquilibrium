use std::ops::{Add, Sub};

use minilp::{ComparisonOp, OptimizationDirection, Problem};
use num::{One, Zero};

use crate::{MixedStrategy, Number};

/// When A has 3 strategies and B has 3 strategies
///
/// Then the payoff matrix for A is `[[a1, a2], [a3, a4]]` and for B is `[[b1, b2], [b3, b4]]`.
///
/// This function returns the mixed strategies for A and B.
pub fn mixed_strategies_3_3(a: &[Number; 4], b: &[Number; 4]) -> ([Number; 2], [Number; 2]) {
    (player_a(a), player_b(b))
}

/// When A has 3 strategies and B has 3 strategies
///
/// Then the payoff matrix for A is `[[a1, a2, a3], [a4, a5, a6], [a7, a8, a9]]`
///
/// And for B is `[[b1, b2, b3], [b4, b5, b6], [b7, b8, b9]]`.
///
/// This function returns the mixed strategies for A and B.
pub fn fast_strategies_3_3(a: &[f64; 9], b: &[f64; 9]) -> (MixedStrategy<f64, 3>, MixedStrategy<f64, 3>) {
    let x = fast_solve3(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8]).unwrap();
    let y = fast_solve3(b[0], b[3], b[6], b[1], b[4], b[7], b[2], b[5], b[8]).unwrap();
    (x, y)
}

fn player_a(a: &[Number; 4]) -> [Number; 2] {
    // (a4 - a2) / (a1 - a2 - a3 + a4)
    let [a1, a2, a3, a4] = a;
    let denominator = a1.sub(a2).sub(a3.clone()).add(a4);
    if denominator.is_zero() {
        //
        if a1 == a2 && a3 == a4 {
            return [Number::fraction(1, 2), Number::fraction(1, 2)];
        }
        //
        else if a1 == a2 && a3 < a4 {
            return [Number::one(), Number::zero()];
        }
        //
        else if a1 < a2 && a3 == a4 {
            return [Number::zero(), Number::one()];
        }
        //
        else if a1.sub(a2) == a4.sub(a3) {
            return [Number::fraction(1, 2), Number::fraction(1, 2)];
        }
        else {
            panic!("Invalid payoff matrix: {:?}", a);
        }
    }
    else {
        let p1 = (a4.sub(a2)) / denominator.clone();
        let p2 = (a1.sub(a3)) / denominator.clone();
        return [p1, p2];
    }
}

fn player_b(b: &[Number; 4]) -> [Number; 2] {
    // (a4 - a2) / (a1 - a2 - a3 + a4)
    let [b1, b2, b3, b4] = b.clone();
    player_a(&[b1, b3, b2, b4])
}

#[rustfmt::skip]
fn fast_solve3(a1: f64, a2: f64, a3: f64, a4: f64, a5: f64, a6: f64, a7: f64, a8: f64, a9: f64) -> Result<MixedStrategy<f64, 3>, minilp::Error> {
    let mut problem = Problem::new(OptimizationDirection::Maximize);
    let x1 = problem.add_var(1.0, (0.0, 1.0));
    let x2 = problem.add_var(1.0, (0.0, 1.0));
    let x3 = problem.add_var(1.0, (0.0, 1.0));
    problem.add_constraint(&[(x1, a4 - a1), (x2, a5 - a2), (x3, a6 - a3)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(x1, a7 - a1), (x2, a8 - a2), (x3, a9 - a3)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(x1, 1.0), (x2, 1.0), (x3, 1.0)], ComparisonOp::Eq, 1.0);
    let solution = problem.solve()?;
    Ok(MixedStrategy { gain: solution.objective(), probability: [solution[x1], solution[x2], solution[x3]] })
}

// #[test]
// fn test() {
//     let a = fast_solve3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
//     println!("{:?}", a);
// }
