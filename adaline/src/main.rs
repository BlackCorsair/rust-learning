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
            weights: random_floats64_vector(inputs),
            threshold: random_float64(),
            inputs: inputs,
            output: 0.0
        }
    }

    fn calculate_output(self, inputs_vector: Vec<Vec<f64>>) -> f64 {
        let inputs_len = inputs_vector[0].len() - 1;
        println!("inputs_len: {:?}", inputs_len);
        assert_eq!((inputs_len), self.weights.len());
        let mut output = 0.0;
        for i in 0..inputs_vector.len() {
            for j in 0..inputs_len {
                output += (inputs_vector[i][j] - self.weights[j]).powi(2) - self.threshold;
            }
        }
        return output;
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
    
    let mut inputs: Vec<Vec<f64>> = Vec::new();
    inputs.push([0.0, 0.2, -0.68, 0.0, 0.2, -0.68, 0.0, 0.2, -0.68].to_vec());
    inputs.push([0.3, 0.3, -0.68, 0.0, 0.2, -0.68, 0.0, 0.2, -0.68].to_vec());

    let _output = adaline.calculate_output(inputs);
    println!("Output: {:?}", _output);
}
