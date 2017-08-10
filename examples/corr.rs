extern crate rustcmb;

use rustcmb::corr::{correlation_check, correlation_distance};
use rustcmb::io::write_2d;
use rustcmb::math;

const SIZE: usize = 64;
const DATA_OUT: &str = "data/examples/out/corr/";

fn main() {

    let field: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];

    let x = math::torus_distance(&field, &[1, 2], &[3, 4]);
    println!("Torus distance between [1, 2] and [3, 4] {}", x);

    let field_check = correlation_check(&field);
    write_2d(&field_check, DATA_OUT, "field_check.dat");

    let field_distance = correlation_distance(&field, &[60, 60]);
    write_2d(&field_distance, DATA_OUT, "field_distance.dat");

    // example for correlation_function
}
