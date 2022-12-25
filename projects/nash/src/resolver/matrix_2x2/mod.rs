use std::ops::{Add, Sub};

use minilp::{ComparisonOp, OptimizationDirection, Problem};
use num::{One, Zero};

use crate::{MixedStrategy, Number};

/// When A has 2 strategies and B has 2 strategies
///
/// Then the payoff matrix for A is `[[a1, a2], [a3, a4]]` and for B is `[[b1, b2], [b3, b4]]`.
///
/// This function returns the mixed strategies for A and B.
pub fn mixed_strategies_2_2(a: &[Number; 4], b: &[Number; 4]) -> ([Number; 2], [Number; 2]) {
    (player_a(a), player_b(b))
}

/// When A has 2 strategies and B has 2 strategies
///
/// Then the payoff matrix for A is `[[a1, a2], [a3, a4]]` and for B is `[[b1, b2], [b3, b4]]`.
///
/// This function returns the mixed strategies for A and B.
pub fn fast_strategies_2_2(a: &[f64; 4], b: &[f64; 4]) -> (MixedStrategy<f64, 2>, MixedStrategy<f64, 2>) {
    let x = fast_solve2(a[0], a[1], a[2], a[3]).unwrap();
    let y = fast_solve2(b[0], b[2], b[1], b[3]).unwrap();
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

fn fast_solve2(a1: f64, a2: f64, a3: f64, a4: f64) -> Result<MixedStrategy<f64, 2>, minilp::Error> {
    let mut problem = Problem::new(OptimizationDirection::Maximize);
    let x1 = problem.add_var(1.0, (0.0, 1.0));
    let x2 = problem.add_var(1.0, (0.0, 1.0));
    problem.add_constraint(&[(x1, a1 - a3), (x2, a2 - a4)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(x1, 1.0), (x2, 1.0)], ComparisonOp::Eq, 1.0);
    let solution = problem.solve()?;
    Ok(MixedStrategy { gain: solution.objective(), probability: [solution[x1], solution[x2]] })
}
