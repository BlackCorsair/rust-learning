mod utils;

use utils::Adaline;
use utils::Data;
use utils::CsvReader;

fn main() {
    let adaline = Adaline::new(2);

    let mut csv_reader = CsvReader::new("src/data.csv");
    &csv_reader.read_content();
    
    let data: Vec<Data> = csv_reader.read_data_from_csv();
    println!("Data: {:#?}", data);

    let output = adaline.calculate_output(&data[0]);
    println!("Output: {:?}", output);
}
