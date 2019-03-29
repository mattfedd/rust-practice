
pub struct Perceptron {
    weights: Vec<f32>,
    bias: f32,
    learning_rate: f32,
}

impl Perceptron {
    pub fn new(dimension: u8) -> Perceptron {
        Perceptron {
            weights: vec![0.0; dimension as usize],
            bias: 0.0,
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

    pub fn get_bias(&self) -> &f32 {
        &self.bias
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
        let mut p = Perceptron::new(2);
        p.set_learning_rate(0.25);
        assert_eq!(p.get_dimension(), 2);
        assert_eq!(p.get_bias(), &0.0);
        assert_eq!(p.get_weights(), &vec![0.0, 0.0]);

        let training_data1 = vec![1.0, 2.0, 0.0];
        // TODO: finish this up
    }
}