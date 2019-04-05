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

    let iter = 10;
    println!("Running training for {} interations", iter);
    let error: [Vec<f64>;2] = adaline.cycle(&data, iter);
    println!("Final error: {:?}", &error);
}
