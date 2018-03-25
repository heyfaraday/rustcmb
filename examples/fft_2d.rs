extern crate rustcmb;

use rustcmb::fourier::fft_2d;
use rustcmb::io::write_2d;
use rustcmb::spectra::gasdev;

#[allow(dead_code)]
const DATA_IN: &str = "../data/in/rust-examples/fft_2d/";
const DATA_OUT: &str = "../data/out/rust-examples/fft_2d/";
const SIZE: usize = 8;

fn main() {
    let mut field: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    gasdev(&field, &mut a_mods, &mut b_mods, 0., 1.);
    println!("a_mods[2] {:?}", a_mods[2]);
    println!("b_mods[4] {:?}", b_mods[4]);

    fft_2d::torus::first_realization(&mut field, &a_mods, &b_mods);
    write_2d(&field, &DATA_OUT, &"first_realization_field.dat");

    fft_2d::torus::second_realization(&mut field, &a_mods, &b_mods);
    write_2d(&field, &DATA_OUT, &"second_realization_field.dat");

    fft_2d::torus::back(&field, &mut a_mods, &mut b_mods);

    println!("a_mods[2] after {:?}", a_mods[2]);
    println!("b_mods[4] after {:?}", b_mods[4]);

    let mut field: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    field[3][3] = 1.;
    println!("field[3] {:?}", field[3]);

    fft_2d::torus::back(&field, &mut a_mods, &mut b_mods);

    fft_2d::torus::first_realization(&mut field, &a_mods, &b_mods);
    write_2d(&field, &DATA_OUT, &"first_realization_field_from_back.dat");

    println!("first realization field[3] after {:?}", field[3]);

    fft_2d::torus::second_realization(&mut field, &a_mods, &b_mods);
    write_2d(&field, &DATA_OUT, &"first_realization_field_from_back.dat");
    println!("second realization field[3] after {:?}", field[3]);
}
