
pub struct DataGenerator;

impl DataGenerator {
    pub fn generate(entry_size: u8, entry_count: usize) -> Vec<Vec<f32>> {
        // TODO:
        vec![Vec::new()]
    }

    pub fn divide_into_train_and_classify_sets(data_set: Vec<Vec<f32>>, training_ratio: f32) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
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