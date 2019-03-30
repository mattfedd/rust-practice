extern crate rand;
use rand::Rng;

pub struct DataGenerator;

impl DataGenerator {
    pub fn generate(entry_size: u8, entry_count: usize) -> Vec<Vec<f32>> {
        // TODO: write tests for this

        // entry size sets the dimension
        // entry count sets the amount of data entries we make
        
        // first we make an internal set of weights and a bias
        let mut rng = rand::thread_rng();
        let rand_upper = 10.0;
        let rand_lower = -10.0;
        let mut internal_weights = Vec::new();
        for i in 0..entry_size {
            internal_weights.push(rng.gen_range(rand_lower, rand_upper));
        }

        let bias = rng.gen::<f32>();

        // now we create a container for all our data
        let mut data_set = Vec::new();

        // loop 
        for i in 0..entry_count {
            // make a random set of values for [x y .. n]
            let mut entry = Vec::new();
            for j in 0..entry_size {
                entry.push(rng.gen_range(rand_lower, rand_upper));
            }
            let is_above = DataGenerator::is_above_hyperplane(&entry, &internal_weights);
            let mut above_res = 0.0;
            if is_above {
                above_res = 1.0;
            }
            entry.push(above_res);
            data_set.push(entry);
        }

        data_set
    }

    // TODO: move this into perceptron file?
    // TODO: write tests for this
    pub fn is_above_hyperplane(input:&Vec<f32>, weights: &Vec<f32>) -> bool {
        // input: [x y z] for 3 dimensional example
        // weights: [a b c]
        // [x y z 1] dot [a b 0 c] -> answer
        // or [x y 1] dot [a b c] -> answer
        // if answer < z then true
        // else false

        let mut result = false;
        let mut answer = 0.0;
        let weight_size = weights.len();
        for i in 0..weight_size-1 {
            answer = answer + input[i] * weights[i];
        }

        // add bias
        answer = answer + weights[weight_size-1];

        let input_val = input[weight_size-1];
        if input_val > answer {
            result = true;
        }

        println!("Input: {:?} ....Result is {}, input try is {}, returning {}", input, answer, input_val, result);

        result
    }

    pub fn divide_into_train_and_classify_sets(data_set: Vec<Vec<f32>>, train_ratio: f32) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
        // TODO:
        (vec![Vec::new()], vec![Vec::new()])
    }

    pub fn is_linearly_separable(data_set: Vec<Vec<f32>>) -> bool {
        // TODO:
        false
    }

    pub fn write_to_file(filename: String, data_set: Vec<Vec<f32>>) {
        // TODO:
    }

    pub fn read_from_file(filename: String) -> Vec<Vec<f32>>{
        // TODO:
        vec![Vec::new()]
    }
}

#[cfg(test)]
mod test {
    use super::DataGenerator;

    #[test]
    fn basics() {
        let data_set = DataGenerator::generate(3, 4);
        assert_eq!(data_set.len(), 4);

        // TODO: expand
    }
}