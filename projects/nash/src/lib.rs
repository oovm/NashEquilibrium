pub use crate::{
    number::Number,
    resolver::{
        matrix_2x2::mixed_strategies_2_2,
        matrix_3x2::{mixed_strategies_2_3, mixed_strategies_3_2},
        matrix_3x3::mixed_strategies_3_3,
    },
};

mod number;

mod resolver;
