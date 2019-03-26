
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

// interpreted as [y = mx + b] for a [y m b] data vector
// or [z = ax + by + c] for [z a b c], etc
#[derive(Debug)]
pub struct TrainingDataElement {
    data: Vec<i32>,
    result: bool,
}

pub struct TrainingData {
    pub header: String,
    pub weights: Vec<i32>,
    pub data: Vec<TrainingDataElement>,
}

pub fn generate_data(vector_size: u8, data_element_count: u32, filename_offset: u8) -> Option<TrainingData> {

    if vector_size == 0 {
        println!("Vector size must be greater than 0");
        return None
    }

    if data_element_count == 0 {
        println!("Data element count must be greater than 0");
        return None
    }

    let range_upper = 10;

    let mut rng = rand::thread_rng();
    
    let mut weight_vec = Vec::new();
    for _i in 0..vector_size-1 {
        weight_vec.push(rng.gen_range(-1*range_upper, range_upper));
    }
    weight_vec.push(1); //for the constant element
    println!("Weight vector is formatted as {}", get_xy_string_from_size(&vector_size));
    println!("Weight vector has {} dimensions plus a bias, is {:?}", vector_size-1, weight_vec);

    let mut data_elements = Vec::new();
    for _i in 0..data_element_count {
        let mut w = Vec::new();
        for _j in 1..vector_size {
            w.push(rng.gen_range(-1*range_upper,range_upper));
        }
        let is_above = is_above_hyperplane(&w, &weight_vec);
        let p = TrainingDataElement{data:w, result:is_above};
        data_elements.push(p)
    }
    
    let mut result = TrainingData {
        header: get_xy_string_from_size(&vector_size),
        weights: weight_vec,
        data: data_elements,
    };

    let mut filename = String::from("training_data_");
    filename.push_str(&filename_offset.to_string());
    filename.push_str(".txt");
    write_data_to_file(&result, filename);

    Some(result)
}

// helper for generate_data
fn is_above_hyperplane(input:&Vec<i32>, weight:&Vec<i32>) -> bool {

    let weight_size = weight.len();
    let mut result = false;

    let mut calc = 0;
    let answer = weight[0];
    for i in 1..weight_size {
        calc = calc + input[i-1] * weight[i];
    }

    if calc > answer {
        result = true;
    }

    println!("Input: {:?} ....Result is {}, actual is {}, returning {}", input, calc, answer, result);

    result
}

fn get_xy_string_from_size(input:&u8) -> String {
    let mut result = String::from("[bias");

    for i in 1..*input {
        result.push(',');
        result.push((i+64) as char);
    }

    result.push(']');

    result
}

// Format (apart from the first line which spoils the weights) is
// input values..., result
pub fn write_data_to_file(input:&TrainingData, filename: String) {
    println!("Filename is {}", filename);
    
}

pub fn read_data_from_file(filename: String) -> Option<TrainingData> {

    None
}