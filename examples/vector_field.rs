extern crate rustcmb;

use rustcmb::corr::{correlation_function, correlation_function_vector_field};
use rustcmb::spectra::{gasdev_max_k, gasdev_exp_k0};
use rustcmb::fourier::fft_2d::torus::{first_realization, second_realization_d_2d_x,
                                      second_realization_d_2d_y};
use rustcmb::diff::d_2d::{d_2d_x, d_2d_y};
use rustcmb::io::{write_1d, write_2d};

const SIZE: usize = 64;
const DATA_OUT: &str = "../data/out/rust-examples/vector_field/";
const MAX_ARG: f64 = SIZE as f64 / 4.;
const SCALE: usize = 20;

fn main() {
    let mut field: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    gasdev_max_k(&field, &mut a_mods, &mut b_mods, 0., 1., 6.);
    // or // MAX_ARG / 2.
    //    gasdev_exp_k0(&field, &mut a_mods, &mut b_mods, 0., 1., MAX_ARG);

    first_realization(&mut field, &a_mods, &b_mods);

    let corr_function = correlation_function(&field, SCALE);
    write_1d(&corr_function[0], DATA_OUT, "corr_function.dat");
    write_1d(&corr_function[1], DATA_OUT, "norm_for_corr_function.dat");

    let mut field_x: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    let mut field_y: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    let mut field_x_second: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    let mut field_y_second: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    d_2d_x(&mut field_x, &a_mods, &b_mods);
    d_2d_y(&mut field_y, &a_mods, &b_mods);
    second_realization_d_2d_x(&mut field_x_second, &a_mods, &b_mods);
    second_realization_d_2d_y(&mut field_y_second, &a_mods, &b_mods);

    let corr_function = correlation_function(&field, SCALE);
    let corr_function_x = correlation_function(&field_x_second, SCALE);
    let corr_function_y = correlation_function(&field_y_second, SCALE);
    write_1d(&corr_function[0], DATA_OUT, "corr_function.dat");
    write_1d(&corr_function[1], DATA_OUT, "norm_for_corr_function.dat");
    write_1d(&corr_function_x[0], DATA_OUT, "corr_function_x.dat");
    write_1d(
        &corr_function_x[1],
        DATA_OUT,
        "norm_for_corr_function_x.dat",
    );
    write_1d(&corr_function_y[0], DATA_OUT, "corr_function_y.dat");
    write_1d(
        &corr_function_y[1],
        DATA_OUT,
        "norm_for_corr_function_y.dat",
    );

    let corr_function_vector_field =
        correlation_function_vector_field(&field_x_second, &field_y_second, SCALE);
    write_1d(
        &corr_function_vector_field[0],
        DATA_OUT,
        "corr_function_vector_field_cos.dat",
    );
    write_1d(
        &corr_function_vector_field[1],
        DATA_OUT,
        "corr_function_vector_field_sin.dat",
    );
    write_1d(
        &corr_function_vector_field[2],
        DATA_OUT,
        "norm_for_corr_function_vector_field.dat",
    );

    write_2d(&field, &DATA_OUT, &"field.dat");
    write_2d(&field_x, &DATA_OUT, &"field_x.dat");
    write_2d(&field_y, &DATA_OUT, &"field_y.dat");
    write_2d(&field_x_second, &DATA_OUT, &"field_x_second.dat");
    write_2d(&field_y_second, &DATA_OUT, &"field_y_second.dat");

    write_2d(&a_mods, &DATA_OUT, &"a_mods.dat");
    write_2d(&b_mods, &DATA_OUT, &"b_mods.dat");
}
