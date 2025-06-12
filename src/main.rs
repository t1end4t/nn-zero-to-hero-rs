use nn_zero_to_hero_rs::micrograd::Value;

fn main() {
    let a = Value::new(-2.0);
    let b = Value::new(4.0);

    let c = a + b;

    println!("{}", c);
}
