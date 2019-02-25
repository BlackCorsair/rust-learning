mod Utils;
use Utils::random_floats64_vector;
use Utils::random_float64;

#[derive(Debug)]
struct Adaline {
    weights: Vec<f64>,
    threshold: f64,
    output: f64,
}
impl Adaline {
    /// Initializes the Adaline struct
    pub fn new (inputs: u8) -> Adaline {
        Adaline{
            weights: random_floats64_vector(inputs),
            threshold: random_float64(),
            output: 0.0
        }
    }

    fn calculate_output (self, data: &Data) -> f64 {
        assert_eq!(data.inputs.len(), self.weights.len());
        let mut output = 0.0;
        
        for input in &data.inputs {
            for weight in &self.weights {
                output += (input - weight).powi(2) - self.threshold;
            }
        }
        
        return output;
    }
}