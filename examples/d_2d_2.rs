extern crate rustcmb;

use rustcmb::fourier::fft_2d;
use rustcmb::diff::d_2d::*;
use rustcmb::io::write_2d;
use rustcmb::spectra::gasdev_max_k;

#[allow(dead_code)]
const DATA_IN: &str = "data/examples/in/d_2d_2/";
const DATA_OUT: &str = "data/examples/out/d_2d_2/";
const SIZE: usize = 8;

fn main() {

    let mut field: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    gasdev_max_k(&field, &mut a_mods, &mut b_mods, 0., 1., SIZE as f64);

    fft_2d::torus::first_realization(&mut field, &a_mods, &b_mods);

    write_2d(&field, &DATA_OUT, &"field.dat");

    let mut field_x: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut field_y: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut a_mods_x: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods_x: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut a_mods_y: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods_y: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    fft_2d::torus::second_realization_d_2d_x(&mut field_x, &a_mods, &b_mods);
    fft_2d::torus::second_realization_d_2d_y(&mut field_y, &a_mods, &b_mods);
    fft_2d::torus::back(&field_x, &mut a_mods_x, &mut b_mods_x);
    fft_2d::torus::back(&field_y, &mut a_mods_y, &mut b_mods_y);

    let mods_x = d_x(&a_mods, &b_mods);
    let mods_y = d_y(&a_mods, &b_mods);

    write_2d(&a_mods, &DATA_OUT, &"a_mods.dat");
    write_2d(&b_mods, &DATA_OUT, &"b_mods.dat");
    write_2d(&a_mods_x, &DATA_OUT, &"a_mods_x.dat");
    write_2d(&b_mods_x, &DATA_OUT, &"b_mods_x.dat");
    write_2d(&mods_x[0], &DATA_OUT, &"a_mods_x_2.dat");
    write_2d(&mods_x[1], &DATA_OUT, &"b_mods_x_2.dat");
    write_2d(&a_mods_y, &DATA_OUT, &"a_mods_y.dat");
    write_2d(&b_mods_y, &DATA_OUT, &"b_mods_y.dat");
    write_2d(&mods_y[0], &DATA_OUT, &"a_mods_y_2.dat");
    write_2d(&mods_y[1], &DATA_OUT, &"b_mods_y_2.dat");
}
