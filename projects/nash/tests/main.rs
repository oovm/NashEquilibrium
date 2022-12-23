use nash::{mixed_strategies_2_2, Number};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let a = &[Number::from(3), Number::from(-1), Number::from(-1), Number::from(0)];
    let b = &[Number::from(2), Number::from(3), Number::from(1), Number::from(0)];
    let (x, y) = mixed_strategies_2_2(a, b);
    println!("{:?}", x);
    println!("{:?}", y);
}
