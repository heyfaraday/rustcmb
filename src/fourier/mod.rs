extern crate rustfft;

use self::rustfft::algorithm::Radix4;
use self::rustfft::FFT;
use self::rustfft::num_complex::Complex;
use self::rustfft::num_traits::Zero;

use std::f64::consts::PI;

pub fn fft_1d_first_realization(field: &mut Vec<f64>, a_mods: &Vec<f64>, b_mods: &Vec<f64>) {

    let size = field.capacity() - 1;

    assert!(field.capacity() == size + 1);
    assert!(a_mods.capacity() == size / 2 + 1);
    assert!(b_mods.capacity() == size / 2 + 1);
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

pub fn fft_1d_second_realization(field: &mut Vec<f64>, a_mods: &Vec<f64>, b_mods: &Vec<f64>) {

    let size = field.capacity() - 1;

    assert!(field.capacity() == size + 1);
    assert!(a_mods.capacity() == size / 2 + 1);
    assert!(b_mods.capacity() == size / 2 + 1);
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

pub fn fft_1d_third_realization(field: &mut Vec<f64>, a_mods: &Vec<f64>, b_mods: &Vec<f64>) {

    let size = field.capacity() - 1;

    assert!(field.capacity() == size + 1);
    assert!(a_mods.capacity() == size / 2 + 1);
    assert!(b_mods.capacity() == size / 2 + 1);
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

//pub fn fft_2d_first_realization(
//    field: &mut Vec<Vec<f64>>,
//    a_mods: &Vec<Vec<f64>>,
//    b_mods: &Vec<Vec<f64>>,
//) {
//
//    let size = field.capacity() - 1;
//
//    assert!(field.capacity() == size + 1);
//    assert!(field[0].capacity() == size + 1);
//    assert!(a_mods.capacity() == size / 2 + 1);
//    assert!(b_mods.capacity() == size / 2 + 1);
//    assert!(a_mods[0].capacity() == size / 2 + 1);
//    assert!(b_mods[0].capacity() == size / 2 + 1);
//    assert!(b_mods[0][0] == 0.); // условно
//    assert!(b_mods[size / 2][size / 2] == 0.);
//    // с модами пока хуй знает
//
//    let mut input_1: Vec<Complex<f64>> = vec![Complex::zero(); size];
//    let mut output_1: Vec<Complex<f64>> = vec![Complex::zero(); size];
//
//    let mut input_2: Vec<Complex<f64>> = vec![Complex::zero(); size];
//    let mut output_2: Vec<Complex<f64>> = vec![Complex::zero(); size];
//
//    for i_mode in 0..(size / 2 + 1) {
//        for j_mode in 0..(size / 2 + 1) {
//            input_1[j] = Complex::new(a_mods[i_mode][j_mode], b_mods[i_mode][j_mode]);
//        }
//
//        let fft = Radix4::new(size, false);
//        fft.process(&mut input, &mut output);
//
//        for j in 0..(size) {
//            output_1[j]
//
//
//
//            for i in 0..(size) {
//                field[i][j] += output_2[i]
//            }
//        }
//    }
//}

pub fn fft_2d_second_realization(
    field: &mut Vec<Vec<f64>>,
    a_mods: &Vec<Vec<f64>>,
    b_mods: &Vec<Vec<f64>>,
) {

    let size = field.capacity() - 1;

    assert!(field.capacity() == size + 1);
    assert!(field[0].capacity() == size + 1);
    // с модами пока хуй знает

    let h = (2. * PI) / (size as f64);

    for i in 0..(size + 1) {
        for j in 0..(size + 1) {
            field[i][j] = 0.;
            for i_mode in 0..(size / 2 + 1) {
                for j_mode in 0..(size / 2 + 1) {
                    let arg = (i * i_mode) as f64 + (j as f64 * (j_mode as f64)); // - (size / 2) as f64));
                    field[i][j] += a_mods[i_mode][j_mode] * (h * arg).cos() +
                        b_mods[i_mode][j_mode] * (h * arg).sin();
                }
            }

        }
    }

}
