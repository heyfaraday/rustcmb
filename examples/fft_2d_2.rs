extern crate rustcmb;

use rustcmb::fourier::fft_2d;

use rustcmb::io::write_2d;

#[allow(dead_code)]
const DATA_IN: &str = "data/examples/in/fft_2d_2/";
const DATA_OUT: &str = "data/examples/out/fft_2d_2/";
const SIZE: usize = 8;

fn main() {
    let mut field_1: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut field_2: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    a_mods[4][0] = 1.;

    fft_2d::torus::first_realization(&mut field_1, &a_mods, &b_mods);
    fft_2d::torus::second_realization(&mut field_2, &a_mods, &b_mods);

    write_2d(&field_1, &DATA_OUT, &"field_1.dat");
    write_2d(&field_2, &DATA_OUT, &"field_2.dat");
}
