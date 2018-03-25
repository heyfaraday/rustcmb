extern crate rustcmb;

use rustcmb::corr::correlation_function;
use rustcmb::spectra::{gasdev, gasdev_max_k, gasdev_exp_k0};
use rustcmb::fourier::fft_2d::torus::first_realization;
use rustcmb::diff::d_2d::{d_2d_xx, d_2d_xy, d_2d_yy};
use rustcmb::io::{write_1d, write_2d};

const SIZE: usize = 64;
const DATA_OUT: &str = "../data/out/rust-examples/tensor_field/";
const MAX_ARG: f64 = SIZE as f64 / 4.;
const SCALE: usize = 20;

fn main() {
    let mut field: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    //    gasdev(&field, &mut a_mods, &mut b_mods, 0., 1.);
    gasdev_max_k(&field, &mut a_mods, &mut b_mods, 0., 1., 4.);
    //    gasdev_exp_k0(&field, &mut a_mods, &mut b_mods, 0., 1., MAX_ARG); // or

    first_realization(&mut field, &a_mods, &b_mods);

    let corr_function = correlation_function(&field, SCALE);
    write_1d(&corr_function[0], DATA_OUT, "corr_function.dat");
    write_1d(&corr_function[1], DATA_OUT, "norm_for_corr_function.dat");

    let mut field_xx: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    let mut field_yy: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    let mut field_xy: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    d_2d_xx(&mut field_xx, &a_mods, &b_mods);
    d_2d_yy(&mut field_yy, &a_mods, &b_mods);
    d_2d_xy(&mut field_xy, &a_mods, &b_mods);

    let mut q: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];
    let mut u: Vec<Vec<f64>> = vec![vec![1.; SIZE + 1]; SIZE + 1];

    for i in 0..SIZE {
        for j in 0..SIZE {
            q[i][j] = field_xx[i][j] - field_yy[i][j];
            u[i][j] = 2. * field_xy[i][j];
        }
    }

    write_2d(&field, &DATA_OUT, &"field.dat");
    write_2d(&q, &DATA_OUT, &"q.dat");
    write_2d(&u, &DATA_OUT, &"u.dat");
}
