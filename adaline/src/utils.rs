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
    learnfactor: f64
}
impl Adaline {
    /// Initializes the Adaline struct
    pub fn new(inputs: u8, learnfactor: f64) -> Adaline {
        Adaline {
            weights: random_floats64_vector(inputs),
            threshold: random_float64(),
            output: 0.0,
            learnfactor: learnfactor
        }
    }

    pub fn calculate_output_per_row(&self, data: &Data) -> f64 {
        assert_eq!(data.inputs.len(), self.weights.len());
        let mut output = 0.0;

        for _i in 0..data.inputs.len() {
            output += data.inputs[_i] * self.weights[_i];
        }

        return output + self.threshold;
    }
/*
def modifyWeights(output, row, weights,  threshold, learnfactor):
    weightsTemp = 0.0
    learnedDiff = learnfactor * (desired_output - output)
    for x in xrange(0, len(weights)):
        weightsTemp = learnedDiff * row[x] # learnedDiff is calculated outside the loop for better performance
        weights[x] = weights[x] + weightsTemp
    threshold = threshold + learnedDiff
    return weights, threshold
 */
    pub fn modify_weights(&mut self, data: &Data) {
        let learned_difference = self.learnfactor * (data.output - self.output);
        self.threshold += learned_difference;
        for i in 0..self.weights.len() {
            self.weights[i] += learned_difference * data.inputs[i]
        }
    }

    pub fn calculate_error(&mut self, data_rows: &Vec<Data>) -> f64 {
        let mut error: f64 = 0.0;
        for data in data_rows {
            let output: f64 = self.calculate_output_per_row(&data);
            error += (data.output - &output).powi(2);
            self.modify_weights(&data);
            
            println!(">>>");
            println!("data.output: {:?}", data.output);
            println!("output: {:?}", output);
            println!("error: {:?}", error);
            println!("weights: {:?}", self.weights[0]);
            println!("<<<");
        }
        return error / data_rows.len() as f64;
    }

    /*
    '''
    Name: training
    Function: makes one cylce of training (from 3 to 5)
    Input: rows, weights, threshold, learnfactor 
    Returns: weights, threshold after the training
'''
def training(rows, weights, threshold, learnfactor):
    for x in rows:
        output = calculateOutputPerRow(x[0:len(weights)], weights, threshold)
        weights, threshold = modifyWeights(output, x, weights, threshold, learnfactor)
    return weights, threshold
    */
    pub fn training(&mut self, cycles: i32, data_rows: &Vec<Data>) -> f64 {
        let mut lowest_error: f64 = 10000.0;
        for _i in 0..cycles {
            let current_error: f64 = self.calculate_error(&data_rows);
            if current_error < lowest_error {
                lowest_error = current_error;
            }
        }
        lowest_error
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
