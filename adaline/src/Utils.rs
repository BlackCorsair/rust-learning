extern crate rand;

use rand::Rng;
use rand::thread_rng;

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