use std::ops::{Add, Mul};

#[derive(Clone)]
pub struct Value {
    pub data: f64,
    pub children: Vec<Value>,
    pub op: String, // operation that produced this node
}

impl Value {
    pub fn new(data: f64) -> Self {
        Self {
            data,
            op: "".to_string(),
            children: Vec::new(),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let new_data = self.data + other.data;
        let mut new_value = Value::new(new_data);
        new_value.op = "+".to_string();
        new_value.children.push(self);
        new_value.children.push(other);
        new_value
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let new_data = self.data * other.data;
        let mut new_value = Value::new(new_data);
        new_value.op = "*".to_string();
        new_value.children.push(self);
        new_value.children.push(other);
        new_value
    }
}
