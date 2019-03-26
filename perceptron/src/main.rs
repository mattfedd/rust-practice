extern crate rand;
use std::io::Error;

mod training_data;
mod perceptron;

fn main() {
    println!("Hello, world!");

    training_data::generate_data(3, 3, 1);

    let p1 = perceptron::Perceptron { weights: Vec::new(), bias: 0 };
    p1.train(String::from("training_data1.txt"));
    p1.classify(Vec::new());
}
