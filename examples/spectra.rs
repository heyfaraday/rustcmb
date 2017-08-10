extern crate rustcmb;

use rustcmb::spectra::{gasdev, gasdev_exp_k0, gasdev_max_k};
use rustcmb::io::write_2d;

const SIZE: usize = 8;
const DATA_OUT: &str = "data/examples/out/spectra/";
const MAX_ARG: f64 = SIZE as f64 / 4.;

fn main() {

    let field: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    gasdev(&field, &mut a_mods, &mut b_mods, 0., 1.);
    write_2d(&a_mods, &DATA_OUT, &"a_mods.dat");
    write_2d(&b_mods, &DATA_OUT, &"b_mods.dat");
    gasdev_exp_k0(&field, &mut a_mods, &mut b_mods, 0., 1., MAX_ARG);
    write_2d(&a_mods, &DATA_OUT, &"exp_a_mods.dat");
    write_2d(&b_mods, &DATA_OUT, &"exp_b_mods.dat");
    gasdev_max_k(&field, &mut a_mods, &mut b_mods, 0., 1., MAX_ARG);
    write_2d(&a_mods, &DATA_OUT, &"max_k_a_mods.dat");
    write_2d(&b_mods, &DATA_OUT, &"max_k_b_mods.dat");
}
