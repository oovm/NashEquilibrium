use minilp::{ComparisonOp, OptimizationDirection, Problem};

use crate::{MixedStrategy, Number};

/// When A has 3 strategies and B has 2 strategies
///
/// Then the payoff matrix for A is `[[a1, a2, a3], [a4, a5, a6]]` and for B is `[[b1, b2, b3], [b4, b5, b6]]`.
///
/// This function returns the mixed strategies for A and B.
pub fn mixed_strategies_3_2(a: &[Number; 6], b: &[Number; 6]) -> ([Number; 3], [Number; 2]) {
    (player_a(a), player_b(b))
}

/// When A has 2 strategies and B has 3 strategies
///
/// Then the payoff matrix for A is `[[a1, a2], [a3, a4], [a5, a6]]` and for B is `[[b1, b2], [b3, b4], [b5, b6]]`.
///
/// This function returns the mixed strategies for A and B.
pub fn mixed_strategies_2_3(a: &[Number; 6], b: &[Number; 6]) -> ([Number; 2], [Number; 3]) {
    (player_b(a), player_a(b))
}

/// When A has 3 strategies and B has 2 strategies
///
/// Then the payoff matrix for A is `[[a1, a2, a3], [a4, a5, a6]]` and for B is `[[b1, b2, b3], [b4, b5, b6]]`.
///
/// This function returns the mixed strategies for A and B.
pub fn fast_strategies_3_2(a: &[f64; 6], b: &[f64; 6]) -> (MixedStrategy<f64, 3>, MixedStrategy<f64, 2>) {
    let x = fast_solve3(a[0], a[1], a[2], a[3], a[4], a[5]).unwrap();
    let y = fast_solve2(b[0], b[1], b[2], b[3], b[4], b[5]).unwrap();
    (x, y)
}

/// When A has 3 strategies and B has 2 strategies
///
/// Then the payoff matrix for A is `[[a1, a2, a3], [a4, a5, a6]]` and for B is `[[b1, b2, b3], [b4, b5, b6]]`.
///
/// This function returns the mixed strategies for A and B.
pub fn fast_strategies_2_3(a: &[f64; 6], b: &[f64; 6]) -> (MixedStrategy<f64, 2>, MixedStrategy<f64, 3>) {
    // 1,2,3,4,5,6 => 1,4,2,5,3,6
    let x = fast_solve3(a[0], a[3], a[1], a[4], a[2], a[5]).unwrap();
    let y = fast_solve2(b[0], b[3], b[1], b[4], b[2], b[5]).unwrap();
    (y, x)
}

fn player_a(_a: &[Number; 6]) -> [Number; 3] {
    todo!()
}

fn player_b(_b: &[Number; 6]) -> [Number; 2] {
    todo!()
}

fn fast_solve3(a1: f64, a2: f64, a3: f64, a4: f64, a5: f64, a6: f64) -> Result<MixedStrategy<f64, 3>, minilp::Error> {
    let mut problem = Problem::new(OptimizationDirection::Maximize);
    let x1 = problem.add_var(1.0, (0.0, 1.0));
    let x2 = problem.add_var(1.0, (0.0, 1.0));
    let x3 = problem.add_var(1.0, (0.0, 1.0));
    problem.add_constraint(&[(x1, a1 - a4), (x2, a2 - a5), (x3, a3 - a6)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(x1, 1.0), (x2, 1.0), (x3, 1.0)], ComparisonOp::Eq, 1.0);
    let solution = problem.solve()?;
    Ok(MixedStrategy { gain: solution.objective(), probability: [solution[x1], solution[x2], solution[x3]] })
}

fn fast_solve2(a1: f64, a2: f64, a3: f64, a4: f64, a5: f64, a6: f64) -> Result<MixedStrategy<f64, 2>, minilp::Error> {
    let mut problem = Problem::new(OptimizationDirection::Maximize);
    let y1 = problem.add_var(1.0, (0.0, 1.0));
    let y2 = problem.add_var(1.0, (0.0, 1.0));
    problem.add_constraint(&[(y1, a2 - a1), (y2, a5 - a4)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(y1, a3 - a1), (y2, a6 - a4)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(y1, 1.0), (y2, 1.0)], ComparisonOp::Eq, 1.0);
    let solution = problem.solve()?;
    Ok(MixedStrategy { gain: solution.objective(), probability: [solution[y1], solution[y2]] })
}
