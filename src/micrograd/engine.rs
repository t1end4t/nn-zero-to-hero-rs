use std::{fmt::Display, ops::Add};

pub struct Value {
    pub data: f32,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value: data: {}", self.data)
    }
}

impl Value {
    pub fn new(input_data: f32) -> Self {
        Value { data: input_data }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Value::new(self.data + rhs.data)
    }
}
