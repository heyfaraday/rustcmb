extern crate rustcmb;

use rustcmb::spectra::{gasdev, gasdev_exp_k0, gasdev_max_k, gasdev_exp_and_sin, return_spectra};
use rustcmb::io::{write_2d, write_1d};
use rustcmb::fourier::fft_2d;

const SIZE: usize = 512;
const DATA_OUT: &str = "data/examples/out/spectra/";
const MAX_ARG: f64 = SIZE as f64 / 8.;
//const MAX_ARG: f64 = 15.;

fn main() {

    println!("MAX_ARG: {}\n", MAX_ARG);

    let mut field: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    gasdev(&field, &mut a_mods, &mut b_mods, 0., 1.);
    //    write_2d(&a_mods, &DATA_OUT, &"a_mods.dat");
    //    write_2d(&b_mods, &DATA_OUT, &"b_mods.dat");
    fft_2d::torus::first_realization(&mut field, &a_mods, &b_mods);
    write_2d(&field, &DATA_OUT, &"gasdev_field.dat");

    gasdev_exp_k0(&field, &mut a_mods, &mut b_mods, 0., 1., MAX_ARG);
    //    write_2d(&a_mods, &DATA_OUT, &"exp_a_mods.dat");
    //    write_2d(&b_mods, &DATA_OUT, &"exp_b_mods.dat");
    fft_2d::torus::first_realization(&mut field, &a_mods, &b_mods);
    write_2d(&field, &DATA_OUT, &"gasdev_exp_k0_field.dat");

    gasdev_max_k(&field, &mut a_mods, &mut b_mods, 0., 1., MAX_ARG);
    //    write_2d(&a_mods, &DATA_OUT, &"max_k_a_mods.dat");
    //    write_2d(&b_mods, &DATA_OUT, &"max_k_b_mods.dat");
    fft_2d::torus::first_realization(&mut field, &a_mods, &b_mods);
    write_2d(&field, &DATA_OUT, &"gasdev_max_k_field.dat");

    gasdev_exp_and_sin(
        &field,
        &mut a_mods,
        &mut b_mods,
        0.,
        20.,
        0.,
        0.,
        1.,
        2.,
        6.,
    );
    //    write_2d(&a_mods, &DATA_OUT, &"exp_and_sin_a_mods.dat");
    //    write_2d(&b_mods, &DATA_OUT, &"exp_and_sin_b_mods.dat");
    fft_2d::torus::first_realization(&mut field, &a_mods, &b_mods);
    write_2d(&field, &DATA_OUT, &"gasdev_exp_and_sin_field.dat");

    let returned_spectra = return_spectra(&field, &a_mods, &b_mods, 100);
    write_1d(&returned_spectra[0], &DATA_OUT, &"returned_exp_k0_spectra.dat");
}
