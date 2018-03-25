extern crate rustcmb;

use rustcmb::fourier::fft_2d;
use rustcmb::diff::d_2d::*;
use rustcmb::io::write_2d;
use rustcmb::spectra::gasdev_max_k;

#[allow(dead_code)]
const DATA_IN: &str = "../data/in/rust-examples/d_2d/";
const DATA_OUT: &str = "../data/out/rust-examples/d_2d/";
const SIZE: usize = 8;

fn main() {
    let mut field: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    gasdev_max_k(&field, &mut a_mods, &mut b_mods, 0., 1., 1.);

    fft_2d::torus::first_realization(&mut field, &a_mods, &b_mods);
    write_2d(&field, &DATA_OUT, &"field.dat");

    let mut field_xx: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut field_yy: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut field_answ: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];

    d_2d_xx(&mut field_xx, &a_mods, &b_mods);
    d_2d_yy(&mut field_yy, &a_mods, &b_mods);

    for i in 0..(SIZE + 1) {
        for j in 0..(SIZE + 1) {
            field_answ[i][j] = -field_xx[i][j] - field_yy[i][j];
        }
    }
    write_2d(&field_answ, &DATA_OUT, &"field_laplace.dat");
}
