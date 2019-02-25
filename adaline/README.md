# Adaline
## Structs
### Adaline

This struct contains the Adaline Neural Network most used variables and the functions it needs to make the calcs.

#### Variables
* _weights: Vec<f64>_: vector or weights with random values (-1.0, 1.0).
* _threshold: f64_: random float value.
* _output: f64_: calculated output.
* _inputs: u_: number of inputs.

#### Functions
* _pub fn new (inputs: u8)_: Initializes the Adaline struct.
* _fn calculate_output (self, data: &Data)_: Given a _Data_ Struct, it will calculate the output with the Adaline.weights and threshold, then return the obtained output for further calculations.
