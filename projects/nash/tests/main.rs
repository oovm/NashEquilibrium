use nash::{fast_strategies_2_2, fast_strategies_3_2, mixed_strategies_2_2, Number};

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

#[test]
fn test2() {
    let a = &[3.0, -1.0, -1.0, 0.0];
    let b = &[2.0, 3.0, 1.0, 0.0];
    let (x, y) = fast_strategies_2_2(a, b);
    println!("{:?}", x);
    println!("{:?}", y);
}

#[test]
fn test3() {
    let a = &[5.0, 10.0, 5.0, 0.0, 5.0, 10.0];
    let b = &[10.0, 15.0, 0.0, 20.0, 5.0, 25.0];
    let (x, y) = fast_strategies_3_2(a, b);
    println!("{:?}", x);
    println!("{:?}", y);
}
