pub use crate::{
    number::Number,
    resolver::{
        matrix_2x2::{fast_strategies_2_2, mixed_strategies_2_2},
        matrix_3x2::{fast_strategies_2_3, fast_strategies_3_2, mixed_strategies_2_3, mixed_strategies_3_2},
        matrix_3x3::{fast_strategies_3_3, mixed_strategies_3_3},
        matrix_4x4::mixed_strategies_4_4,
        MixedStrategy,
    },
};

mod number;

mod resolver;
