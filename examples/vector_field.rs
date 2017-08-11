extern crate rustcmb;

use rustcmb::corr::{correlation_function, correlation_function_vector_field};
use rustcmb::spectra::{gasdev_max_k, gasdev_exp_k0};
use rustcmb::fourier::fft_2d::first_realization;
use rustcmb::diff::d_2d::{d_2d_x, d_2d_y};
use rustcmb::io::write_1d;

const SIZE: usize = 64; // 128
const DATA_OUT: &str = "data/examples/out/vector_field/";
const MAX_ARG: f64 = SIZE as f64 / 4.;
const SCALE: usize = 20;

fn main() {

    let mut field: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    gasdev_max_k(&field, &mut a_mods, &mut b_mods, 0., 1., 4.);
    // or // MAX_ARG / 2.
    gasdev_exp_k0(&field, &mut a_mods, &mut b_mods, 0., 1., MAX_ARG);

    first_realization(&mut field, &a_mods, &b_mods);

    let corr_function = correlation_function(&field, SCALE);
    write_1d(&corr_function[0], DATA_OUT, "corr_function.dat");
    write_1d(&corr_function[1], DATA_OUT, "norm_for_corr_function.dat");

    let mut field_x: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    let mut field_y: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    d_2d_x(&mut field_x, &a_mods, &b_mods);
    d_2d_y(&mut field_y, &a_mods, &b_mods);

    let corr_function_vector_field = correlation_function_vector_field(&field_x, &field_y, SCALE);
    write_1d(
        &corr_function_vector_field[0],
        DATA_OUT,
        "corr_function_vector_field.dat",
    );
    write_1d(
        &corr_function_vector_field[1],
        DATA_OUT,
        "norm_for_corr_function_vector_field.dat",
    );
}
