extern crate rustcmb;

use rustcmb::fourier::{fft_1d_first_realization, fft_1d_second_realization,
                       fft_1d_third_realization};
use rustcmb::io::write_1d;

const DATA_IN: &str = "data/examples/int/fft_1d/";
const DATA_OUT: &str = "data/examples/out/fft_1d/";
const SIZE: usize = 8;

fn main() {

    let mut simple_field: Vec<f64> = vec![0.; SIZE + 1];
    let mut a_mods: Vec<f64> = vec![0.; SIZE / 2 + 1];
    let mut b_mods: Vec<f64> = vec![0.; SIZE / 2 + 1];

    println!("simple_field: {:?}", simple_field);
    println!("a_mods: {:?}", a_mods);
    println!("b_mods: {:?}", b_mods);

    a_mods[0] = 1.;
    a_mods[1] = 2.;
    a_mods[2] = 3.;
    a_mods[4] = -4.;
    b_mods[1] = 1.;
    b_mods[2] = 3.;
    b_mods[3] = -5.;

    fft_1d_first_realization(&mut simple_field, &a_mods, &b_mods);
    println!("simple_field after first: {:?}", simple_field);

    fft_1d_second_realization(&mut simple_field, &a_mods, &b_mods);
    println!("simple_field after second: {:?}", simple_field);

    fft_1d_third_realization(&mut simple_field, &a_mods, &b_mods);
    println!("simple_field after third: {:?}", simple_field);

    write_1d(&simple_field, &DATA_OUT, &"f.dat");
}
