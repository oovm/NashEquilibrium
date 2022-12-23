use crate::Number;

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

fn player_a(a: &[Number; 6]) -> [Number; 3] {
    let [a1, a2, a3, a4, a5, a6] = a;
}

fn player_b(b: &[Number; 6]) -> [Number; 2] {
    todo!()
}
