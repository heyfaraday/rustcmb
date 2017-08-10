extern crate rustcmb;

use rustcmb::corr::{correlation_function, correlation_check, correlation_distance,
                    correlation_function_vector_field};
use rustcmb::io::{write_2d, write_1d};
use rustcmb::math;

const SIZE: usize = 64;
const DATA_OUT: &str = "data/examples/out/corr/";

fn main() {

    let field: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];

    let x = math::torus_distance(&field, &[1, 2], &[3, 4]);
    println!("Torus distance between [1, 2] and [3, 4] {}", x);

    let field_check = correlation_check(&field);
    write_2d(&field_check, DATA_OUT, "field_check.dat");

    let field_distance = correlation_distance(&field, &[60, 60]);
    write_2d(&field_distance, DATA_OUT, "field_distance.dat");

    let answer_for_simple_correlation = correlation_function(&field, 20);
    write_1d(
        &answer_for_simple_correlation[0],
        DATA_OUT,
        "correlation_function.dat",
    );
    write_1d(
        &answer_for_simple_correlation[1],
        DATA_OUT,
        "norm_for_correlation_function.dat",
    );

    let answer_for_simple_correlation_vector_field =
        correlation_function_vector_field(&field, &field, 20);
    write_1d(
        &answer_for_simple_correlation_vector_field[0],
        DATA_OUT,
        "correlation_function_vector_field.dat",
    );
    write_1d(
        &answer_for_simple_correlation_vector_field[1],
        DATA_OUT,
        "norm_for_correlation_function_vector_field.dat",
    );
}
