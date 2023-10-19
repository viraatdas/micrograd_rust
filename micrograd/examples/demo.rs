extern crate micrograd; // this assumes your library is named "micrograd"

use micrograd::engine::Value; // or whatever the path is to the Value struct

fn main() {
    let v1 = Value::new(1.0);
    let v2 = Value::new(2.0);
    let v3 = v1 + v2;
    println!("Sum is: {}", v3.data);

    let v4 = v1 * v2;
    println!("Product is: {}", v4.data);
}
