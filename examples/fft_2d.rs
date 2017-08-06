extern crate rustcmb;
extern crate rand;

use rustcmb::io::write_2d;
use rustcmb::fourier::fft_2d_second_realization;
use rustcmb::spectra::gasdev;

const DATA_IN: &str = "data/examples/int/fft_2d/";
const DATA_OUT: &str = "data/examples/out/fft_2d/";
const SIZE: usize = 64;

fn main() {

    let mut field: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE / 2 + 1]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE / 2 + 1]; SIZE / 2 + 1];

    a_mods[1][2] = 1.;
    b_mods[1][3] = 0.;
    a_mods[1][5] = 3.;

    gasdev(&mut a_mods, &mut b_mods, 0., 1.);

    fft_2d_second_realization(&mut field, &a_mods, &b_mods);

    write_2d(&field, &DATA_OUT, &"f.dat");


}
