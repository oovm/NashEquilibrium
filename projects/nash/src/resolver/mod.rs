pub mod matrix_2x2;
pub mod matrix_3x2;
pub mod matrix_3x3;
pub mod matrix_4x4;

#[derive(Debug)]
pub struct MixedStrategy<T, const N: usize> {
    pub gain: T,
    pub probability: [T; N],
}
