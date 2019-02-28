mod utils;

use utils::Adaline;
use utils::CsvReader;
use utils::Data;

fn main() {
    let adaline = Adaline::new(8);

    //let mut csv_reader = CsvReader::new("src/data.csv");
    let mut csv_reader = CsvReader::new("src/RandomizedDataTraining.csv");
    &csv_reader.read_content();

    let data: Vec<Data> = csv_reader.read_data_from_csv();

    let error = adaline.calculate_error(data);
    println!("Error calculated: {:?}", error);
}
