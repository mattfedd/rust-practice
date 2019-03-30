


// example, 3 dimensions
// z = ax + by + c hyperplane
// - if above = true
// - if below = false

// weights: [a, b, c]

// classifying inputs: [x, y, z] -> bool
// training inputs: [x, y, z, bool]

// perceptron stores: internal [a, b, c] that it modifies with each training input

// perceptron takes in x y z and has its own internal hyperplane that its comparing to
// aka
// perceptron has a b c guesses
// perceptron does z = ax + by + c using the input x and y (not input z)
// perceptron compares that z against input z -> above/below for classification

// aka
// input: [x y z]
// weights: [a b c]
// [x y z 1] dot [a b 0 c] -> answer
// or [x y 1] dot [a b c] -> answer
// if answer < z then true
// else false

pub struct Perceptron {
    weights: Vec<f32>,
    learning_rate: f32,
}

impl Perceptron {
    pub fn new(dimension: u8) -> Perceptron {
        Perceptron {
            weights: vec![0.0; dimension as usize],
            learning_rate: 0.1,
        }
    }

    pub fn set_learning_rate(&mut self, rate: f32) {
        self.learning_rate = rate;
    }

    pub fn get_dimension(&self) -> u8 {
        self.weights.len() as u8
    }

    pub fn get_weights(&self) -> &Vec<f32> {
        &self.weights
    } 

    // TODO: see if can remove
    pub fn get_bias(&self) -> &f32 {
        &self.weights[&self.weights.len()-1]
    }

    pub fn train_single(&self, data_element: &Vec<f32>) {
        // TODO: 
    }

    pub fn train_set(&self, data_set: &Vec<Vec<f32>>) {
        for element in data_set {
            self.train_single(&element);
        }
    }

    // classify data should be weights.len()+1 to accomodate the true/false at the end
    pub fn classify_single(&self, data_element: &Vec<f32>) -> bool {
        // TODO: 
        false
    }

    // not sure how this should be used
    // pub fn classify_set(&self, data_set: &Vec<Vec<f32>>) -> Vec<bool> {

    // }
}

#[cfg(test)]
mod test {
    use super::Perceptron;

    #[test]
    fn basics() {
         // z = ay + bx + c
         // input data to be classified is [x, y]
         // perceptron weights are [a, b]
         // bias is c
        let mut p = Perceptron::new(3);
        p.set_learning_rate(0.25);
        assert_eq!(p.get_dimension(), 3);
        assert_eq!(p.get_bias(), &0.0);
        assert_eq!(p.get_weights(), &vec![0.0, 0.0, 0.0]);

        let training_data1 = vec![1.0, 2.0, 0.0];
        // TODO: finish this up
    }
}