use std::io::*;
fn main() {
    let name: String = String::from(string_input("hey, what's your name?"));
    println!("Oh, hi {name}", name=&name);
}

fn string_input(input_string: &'static str) -> String {
    let mut ret_string: String = String::new();
    print!("{:?} ", input_string);
    stdout().flush().expect("Weeee");
    stdin().read_line(&mut ret_string)
        .expect("Oh Oh...");
    return ret_string;
}