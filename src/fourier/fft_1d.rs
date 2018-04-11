extern crate rustfft;

use self::rustfft::algorithm::Radix4;
use self::rustfft::FFT;
use self::rustfft::num_complex::Complex;
use self::rustfft::num_traits::Zero;

use std::f64::consts::PI;

pub fn first_realization(field: &mut Vec<f64>, a_mods: &Vec<f64>, b_mods: &Vec<f64>) {
    let size = field.len() - 1;

    assert!(field.len() == size + 1);
    assert!(a_mods.len() == size / 2 + 1);
    assert!(b_mods.len() == size / 2 + 1);
    assert!(b_mods[0] == 0.);
    assert!(b_mods[size / 2] == 0.);

    let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
    let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

    for i in 0..(size / 2 + 1) {
        input[i] = Complex::new(a_mods[i], 0.);
    }

    let fft = Radix4::new(size, false);
    fft.process(&mut input, &mut output);

    for i in 0..(size) {
        field[i] = output[i].re;
    }
    field[size] = output[0].re;

    for i in 1..(size / 2) {
        input[i] = Complex::new(b_mods[i], 0.);
    }

    let fft = Radix4::new(size, false);
    fft.process(&mut input, &mut output);

    for i in 0..(size) {
        field[i] -= output[i].im;
    }
    field[size] -= output[0].im;
}

pub fn second_realization(field: &mut Vec<f64>, a_mods: &Vec<f64>, b_mods: &Vec<f64>) {
    let size = field.len() - 1;

    assert!(field.len() == size + 1);
    assert!(a_mods.len() == size / 2 + 1);
    assert!(b_mods.len() == size / 2 + 1);
    assert!(b_mods[0] == 0.);
    assert!(b_mods[size / 2] == 0.);

    let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
    let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

    for i in 0..(size / 2 + 1) {
        input[i] = Complex::new(a_mods[i], b_mods[i]);
    }

    let fft = Radix4::new(size, false);
    fft.process(&mut input, &mut output);

    for i in 0..(size) {
        field[i] = output[i].re;
    }
    field[size] = output[0].re;
}

pub fn third_realization(field: &mut Vec<f64>, a_mods: &Vec<f64>, b_mods: &Vec<f64>) {
    let size = field.len() - 1;

    assert!(field.len() == size + 1);
    assert!(a_mods.len() == size / 2 + 1);
    assert!(b_mods.len() == size / 2 + 1);
    assert!(b_mods[0] == 0.);
    assert!(b_mods[size / 2] == 0.);

    let h = (2. * PI) / (size as f64);

    for i in 0..(size + 1) {
        field[i] = 0.;
        for i_mode in 0..(size / 2 + 1) {
            let arg = (i * i_mode) as f64;
            field[i] += a_mods[i_mode] * (h * arg).cos() + b_mods[i_mode] * (h * arg).sin();
        }
    }
}

pub fn back(field: &Vec<f64>, a_mods: &mut Vec<f64>, b_mods: &mut Vec<f64>) {
    let size = field.len() - 1;

    assert!(field.len() == size + 1);
    assert!(a_mods.len() == size / 2 + 1);
    assert!(b_mods.len() == size / 2 + 1);
    assert!(b_mods[0] == 0.);
    assert!(b_mods[size / 2] == 0.);

    let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
    let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

    for i in 0..(size) {
        input[i] = Complex::new(field[i], 0.);
    }

    let fft = Radix4::new(size, true);
    fft.process(&mut input, &mut output);

    a_mods[0] = output[0].re / (size as f64);
    a_mods[size / 2] = output[size / 2].re / (size as f64);
    for i in 1..(size / 2) {
        a_mods[i] = output[i].re / (size as f64) * 2.;
        b_mods[i] = output[i].im / (size as f64) * 2.;
    }
}
