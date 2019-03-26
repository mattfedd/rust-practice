use crate::training_data::read_data_from_file;

pub struct Perceptron {
    pub weights: Vec<i32>,
    pub bias: i32,
}

impl Perceptron {

    pub fn new(dimension:usize) -> Perceptron {
        Perceptron {
            weights: vec![0; dimension],
            bias: 0,
        }
    }

    pub fn train(&self, filename: String) -> Perceptron {
        let current_size = self.weights.len();
        let data = read_data_from_file(filename).unwrap();

        println!("{}", data.header);
        println!("Actual weights are {:?}", data.weights);
        let elements = data.data;

        let new_weights = vec![0; current_size];
        let new_bias = 0;

        for item in elements {
            // train!

        }

        Perceptron {
            weights: new_weights,
            bias: new_bias,
        }
    }

    pub fn classify(&self, data: Vec<i32>) -> bool {


        false
    }
}