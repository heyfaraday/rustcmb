extern crate rustcmb;

use rustcmb::fourier::fft_1d;
use rustcmb::io::write_1d;

#[allow(dead_code)]
const DATA_IN: &str = "data/examples/in/fft_1d/";
const DATA_OUT: &str = "data/examples/out/fft_1d/";
const SIZE: usize = 8;

fn main() {

    let mut simple_field: Vec<f64> = vec![0.; SIZE + 1];
    let mut a_mods: Vec<f64> = vec![0.; SIZE / 2 + 1];
    let mut b_mods: Vec<f64> = vec![0.; SIZE / 2 + 1];

    a_mods[0] = 1.;
    a_mods[1] = 2.;
    a_mods[2] = 123.;
    a_mods[3] = 3.;
    a_mods[4] = 5.;
    // b_mods[0] = 1.;
    b_mods[1] = 100.;
    b_mods[2] = 223.;
    b_mods[3] = 34.;
    // b_mods[4] = 100.;

    println!("simple_field: {:?}", simple_field);
    println!("a_mods: {:?}", a_mods);
    println!("b_mods: {:?}", b_mods);

    fft_1d::first_realization(&mut simple_field, &a_mods, &b_mods);
    println!("simple_field after first: {:?}", simple_field);

    fft_1d::second_realization(&mut simple_field, &a_mods, &b_mods);
    println!("simple_field after second: {:?}", simple_field);

    fft_1d::third_realization(&mut simple_field, &a_mods, &b_mods);
    println!("simple_field after third: {:?}", simple_field);

    write_1d(&simple_field, &DATA_OUT, &"f.dat");

    fft_1d::back(&simple_field, &mut a_mods, &mut b_mods);
    println!("a_mods: {:?}", a_mods);
    println!("b_mods: {:?}", b_mods);
}
