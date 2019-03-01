mod utils;

use utils::Adaline;
use utils::CsvReader;
use utils::Data;

fn main() {
    let mut adaline = Adaline::new(8, 0.05);

    //let mut csv_reader = CsvReader::new("src/data.csv");
    let mut csv_reader = CsvReader::new("src/RandomizedDataTraining.csv");
    &csv_reader.read_content();

    let data: Vec<Data> = csv_reader.read_data_from_csv();

    let error = adaline.calculate_error(&data);
    println!("Error calculated: {:?}", error);
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
fn full_test() {
    // setup
    let mut adaline = Adaline::new(8, 0.05);
    let mut csv_reader = CsvReader::new("src/RandomizedDataTraining.csv");
    &csv_reader.read_content();
    let data_list: Vec<Data> = csv_reader.read_data_from_csv();
    
    // check the weights
    println!("unmodified weights: {:?}", &adaline.weights);
    adaline.modify_weights(&data_list[0]);
    
    // run!
    let error: f64 = adaline.training(10000, &data_list);
    println!("Final error: {:?}", &error);
}