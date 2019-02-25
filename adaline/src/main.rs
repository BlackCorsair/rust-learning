use rand::Rng;
use rand::thread_rng;

#[derive(Debug)]
struct Adaline {
    weights: Vec<f64>,
    threshold: f64,
    output: f64,
    inputs: u8
}
impl Adaline {
    fn new(inputs: u8) -> Adaline {
        Adaline{
            threshold: random_float64(),
            inputs: inputs,
            output: 0.0,
            weights: random_floats64_vector(inputs)
        }
    }
}

fn random_floats64_vector(lenght: u8) -> Vec<f64> {
    let mut floats: Vec<f64> = Vec::new();
    for _i in 0..lenght {
        &floats.push(random_float64());
    }
    floats
}


fn random_float64() -> f64 {
    let mut rng = thread_rng();
    return rng.gen_range(-1.0, 1.0);
}

fn main() {
    let adaline = Adaline::new(8);
    println!("Threshold: {threshold},
        \n weights: {weights:#?}",
        threshold=adaline.threshold,
        weights=adaline.weights);
}
