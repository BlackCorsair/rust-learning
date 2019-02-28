extern crate rand;

use rand::thread_rng;
use rand::Rng;

use std::fs::File;
use std::io::Read;
use std::string::String;

#[derive(Debug)]
pub struct CsvReader {
    file_name: String,
    file: File,
    raw_content: String,
    data: Vec<f64>,
}

impl CsvReader {
    pub fn new(file_name: &'static str) -> CsvReader {
        CsvReader {
            file_name: String::from(file_name),
            file: File::open(file_name).expect("Couldn't open the file specified."),
            raw_content: String::new(),
            data: Vec::new(),
        }
    }

    pub fn read_content(&mut self) {
        self.file
            .read_to_string(&mut self.raw_content)
            .expect("Error reading file content.");
    }

    pub fn read_data_from_csv(&mut self) -> Vec<Data> {
        let mut data: Vec<Data> = Vec::new();

        for line in self.raw_content.lines() {
            let mut inputs = line
                .split(',')
                .filter_map(|s| s.parse::<f64>().ok())
                .collect::<Vec<_>>();
            let output: f64 = inputs.pop().expect("Ops");
            data.push(Data {
                inputs: inputs,
                output: output,
            });
        }
        return data;
    }
}

#[derive(Debug)]
pub struct Adaline {
    pub weights: Vec<f64>,
    threshold: f64,
    output: f64,
}
impl Adaline {
    /// Initializes the Adaline struct
    pub fn new(inputs: u8) -> Adaline {
        Adaline {
            weights: random_floats64_vector(inputs),
            threshold: random_float64(),
            output: 0.0,
        }
    }

    pub fn calculate_output(&self, data: &Data) -> f64 {
        assert_eq!(data.inputs.len(), self.weights.len());
        let mut output = 0.0;

        for input in &data.inputs {
            for weight in &self.weights {
                output += (input - weight).powi(2) + self.threshold;
            }
        }

        return output;
    }

    pub fn calculate_error(&self, data_rows: Vec<Data>) -> f64 {
        let mut error: f64 = 0.0;
        for data in &data_rows {
            let output: f64 = self.calculate_output(&data);
            error += (data.output - &output).powi(2);
        }
        return error / data_rows.len() as f64;
    }

    pub fn modify_weights(&mut self, data: &Data, learnfactor: f64) {
        let learned_difference = learnfactor * (data.output - self.output);
        self.threshold += learned_difference;
        for i in 0..self.weights.len() {
            self.weights[i] += &learned_difference * &data.inputs[i]
        }
    }
}

#[derive(Debug)]
pub struct Data {
    pub inputs: Vec<f64>,
    pub output: f64,
}

pub fn random_floats64_vector(lenght: u8) -> Vec<f64> {
    let mut floats: Vec<f64> = Vec::new();
    for _i in 0..lenght {
        &floats.push(random_float64());
    }
    floats
}

pub fn random_float64() -> f64 {
    let mut rng = thread_rng();
    return rng.gen_range(-1.0, 1.0);
}
