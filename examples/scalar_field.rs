extern crate rustcmb;

use rustcmb::spectra::{exp_and_sin, gasdev_exp_and_sin, return_spectra};
use rustcmb::io::{write_1d, write_2d};
use rustcmb::corr::correlation_function;
use rustcmb::fourier::fft_2d;

const SIZE: usize = 64;
const DATA_OUT: &str = "../data/out/rust-examples/scalar_field/";

fn main() {
    let mut field: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    println!("{}, {}, {}", SIZE, field.len() - 1, field.len() - 1);

    //    gasdev(&field, &mut a_mods, &mut b_mods, 0., 1.);
    gasdev_exp_and_sin(&field, &mut a_mods, &mut b_mods, 0., 5., 0., 0., 1., 2., 6.);

    fft_2d::torus::first_realization(&mut field, &a_mods, &b_mods);

    write_2d(&field, &DATA_OUT, &"empty_field.dat");
    write_2d(&a_mods, &DATA_OUT, &"empty_a_mods.dat");
    write_2d(&b_mods, &DATA_OUT, &"empty_b_mods.dat");

    let number_of_bins = 32;

    let mut dispersion: Vec<f64> = vec![0.; number_of_bins];

    let spectra = return_spectra(&field, &a_mods, &b_mods, number_of_bins);
    let corr_function = correlation_function(&field, number_of_bins);

    let mut sum_of_norm = 0.;
    let mut sum_of_corr_norm = 0.;

    for i in 0..number_of_bins {
        let arg = (i as f64) / (number_of_bins as f64) * (SIZE as f64 / (2. as f64).sqrt());

        dispersion[i] = exp_and_sin(&SIZE, &arg, &5., &0., &0., &1., &2., &6.).powi(2);

        sum_of_norm += spectra[1][i];
        sum_of_corr_norm += corr_function[1][i];
    }

    println!("{}. {}", sum_of_norm, sum_of_corr_norm);
    println!("{}, {}", SIZE * SIZE, SIZE * SIZE * (SIZE * SIZE - 1) / 2);

    write_1d(&spectra[0], &DATA_OUT, &"spectra.dat");
    write_1d(&spectra[1], &DATA_OUT, &"norm_for_spectra.dat");
    write_1d(&dispersion, &DATA_OUT, &"dispersion.dat");

    write_1d(&corr_function[0], DATA_OUT, "correlation_function.dat");
    write_1d(
        &corr_function[1],
        DATA_OUT,
        "norm_for_correlation_function.dat",
    );
}
