mod Data;
use Data::Data;

mod Adaline;
use Adaline::Adaline;

use std::io::Read;
use std::fs::File;
use std::string::String;

#[derive(Debug)]
struct CsvReader {
    file_name: String,
    file: File,
    raw_content: String,
    data: Vec<f64>,
}

impl CsvReader {
    fn new (file_name: &'static str) -> CsvReader {
        CsvReader {
            file_name: String::from(file_name),
            file: File::open(file_name)
            .expect("Couldn't open the file specified."),
            raw_content: String::new(),
            data: Vec::new(),
        }
    }
    
    fn read_content (&mut self){
        self.file.read_to_string(&mut self.raw_content)
        .expect("Error reading file content.");
    }

    fn read_data_from_csv(&mut self) -> Vec<Data>{
        let mut data: Vec<Data> = Vec::new();

        for line in self.raw_content.lines() {
            let mut inputs = line.split(',')
                                .filter_map(|s| s.parse::<f64>().ok())
                                .collect::<Vec<_>>();
            let output: f64 = inputs.pop().expect("Ops");
            data.push(
                Data {
                    inputs: inputs,
                    output: output
                }
            );
        }
        return data
    }
}

fn main() {
    let adaline = Adaline::new(2);

    let mut csv_reader = CsvReader::new("src/data.csv");
    &csv_reader.read_content();
    
    let data: Vec<Data> = csv_reader.read_data_from_csv();
    println!("Data: {:#?}", data);

    let output = adaline.calculate_output(&data[0]);
    println!("Output: {:?}", output);

}
