extern crate rand;

use rand::thread_rng;
use rand::Rng;

use std::fs::File;
use std::io::Read;
use std::string::String;

use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

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
    /*
        Public functions, only need / use these
     */
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

    pub fn cycle(&mut self, data_rows: &Vec<Data>, cycles: i32) -> [Vec<f64>;2]{
        let mut error_training: Vec<f64> = Vec::new();
        let mut error_validation: Vec<f64> = Vec::new();

        for _i in 0..cycles {
            self.training(data_rows);
            &error_training.push(self.calculate_error(data_rows));
            &error_validation.push(self.calculate_error(data_rows));
        }

        return [error_training, error_validation];
    }

    /*
        Private functions. Do NOT atemp to use this on your own.
     */

    fn modify_weights(&mut self, data: &Data) {
        let learned_difference = self.learnfactor * (data.output - self.output);
        self.threshold += learned_difference;
        for i in 0..self.weights.len() {
            self.weights[i] += learned_difference * data.inputs[i]
        }
    }

    fn calculate_error(&mut self, data_rows: &Vec<Data>) -> f64 {
        let mut error: f64 = 0.0;
        for data in data_rows {
            let output: f64 = self.calculate_output_per_row(&data);
            error += (data.output - &output).powi(2);
            self.modify_weights(&data);
        }
        return error / data_rows.len() as f64;
    }


    fn training(&mut self, data_rows: &Vec<Data>) {
        for data in data_rows {
            self.output = self.calculate_output_per_row(data);
            self.modify_weights(data);
        }
    }

    fn write_errors_in_csv(self, errors: [Vec<f64>;2]) {
        let training = Path::new("error_training.csv");

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut training_file = match File::create(&training) {
            Err(why) => panic!("couldn't create error_training: {}",
               why.description()),
            Ok(file) => file,
        };

        for i in 0..errors[0].len() {
            let line = &format!("{},{}\n", errors[0][i],errors[1][i]);
            // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
            match training_file.write_all(line.as_bytes()) {
                Err(why) => {
                    panic!("couldn't write to training_file: {}",
                     why.description())
                },
                Ok(_) => (),
            }
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

#[test]
fn test_random_float() {
    let rnd: f64 = random_float64();
    assert!(rnd <= 1.0, "weight should be less or equal than 1.0");
    assert!(rnd >= -1.0, "weight should be greater or equal than -1.0");
}
#[test]
fn test_random_floats_vector() {
    let rnd_floats64: Vec<f64> = random_floats64_vector(8);
    for rnd in rnd_floats64 {
        assert!(rnd <= 1.0, "weight should be less or equal than 1.0");
        assert!(rnd >= -1.0, "weight should be greater or equal than -1.0");
    }
}
#[test]
fn test_modify_weights() {
    let mut adaline = Adaline::new(8, 0.05);
    let mut csv_reader = CsvReader::new("src/RandomizedDataTraining.csv");
    &csv_reader.read_content();
    let data_list: Vec<Data> = csv_reader.read_data_from_csv();
    println!("unmodified weights: {:?}", &adaline.weights);
    adaline.modify_weights(&data_list[0]);
    println!("weights: {:?}", &adaline.weights);
}
#[test]
fn test_calculate_output() {
    // setup
    let adaline = Adaline::new(8, 0.05);
    let mut csv_reader = CsvReader::new("src/RandomizedDataTraining.csv");
    &csv_reader.read_content();
    let data_list: Vec<Data> = csv_reader.read_data_from_csv();

    let output: f64 = adaline.calculate_output_per_row(&data_list[0]);
    assert!(output <= 1.0, "calculated output should be less or equal than 1.0");
    assert!(output >= -1.0, "calculated output should be greater or equal than -1.0");

}
#[test]
fn full_test() {
    // setup
    let mut adaline = Adaline::new(8, 0.05);
    let mut csv_reader = CsvReader::new("src/RandomizedDataTraining.csv");
    &csv_reader.read_content();
    let data_list: Vec<Data> = csv_reader.read_data_from_csv();

    // run!
    let iter = 1000;
    println!("Running training for {} interations", iter);
    let error: [Vec<f64>;2] = adaline.cycle(&data_list, iter);
    assert!(error[0].len() > 1, "error_training should contain more than one value");
    assert!(error[1].len() > 1, "error_validation should contain more than one value");
    adaline.write_errors_in_csv(error);    
}