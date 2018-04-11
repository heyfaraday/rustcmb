extern crate rustfft;

use err::{nyquist_assert_2d, size_assert_2d};
use math::j_mode_normalized;

use self::rustfft::algorithm::Radix4;
use self::rustfft::FFT;
use self::rustfft::num_complex::Complex;
use self::rustfft::num_traits::Zero;

use std::f64::consts::PI;

pub fn first_realization(
    field: &mut Vec<Vec<f64>>,
    a_mods: &Vec<Vec<f64>>,
    b_mods: &Vec<Vec<f64>>,
) {
    let size = field.len() - 1;

    size_assert_2d(&field, &a_mods, &b_mods);
    nyquist_assert_2d(&field, &a_mods, &b_mods);

    let mut temporary: Vec<Vec<Complex<f64>>> = vec![vec![Complex::zero(); size]; size / 2 + 1];

    for i_mode in 0..(size / 2 + 1) {
        let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

        for j_mode in 0..size {
            input[j_mode] = Complex::new(a_mods[i_mode][j_mode], b_mods[i_mode][j_mode]);
        }

        let fft = Radix4::new(size, false);
        fft.process(&mut input, &mut output);

        for j in 0..size {
            temporary[i_mode][j] = output[j];
        }
    }

    for j in 0..size {
        let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

        input[0] = Complex::new(temporary[0][j].re, 0.);
        input[size / 2] = Complex::new(temporary[size / 2][j].re, 0.);
        for i_mode in 1..(size / 2) {
            input[i_mode] = Complex::new(temporary[i_mode][j].re, temporary[i_mode][j].im);
        }

        let fft = Radix4::new(size, false);
        fft.process(&mut input, &mut output);

        for i in 0..size {
            field[i][j] = output[i].re;
        }
        field[size][j] = output[0].re;
    }
    for i in 0..(size + 1) {
        field[i][size] = field[i][0];
    }
}

pub fn second_realization(
    field: &mut Vec<Vec<f64>>,
    a_mods: &Vec<Vec<f64>>,
    b_mods: &Vec<Vec<f64>>,
) {
    let size = field.len() - 1;

    size_assert_2d(&field, &a_mods, &b_mods);
    nyquist_assert_2d(&field, &a_mods, &b_mods);

    let h = (2. * PI) / (size as f64);

    for i in 0..(size + 1) {
        for j in 0..(size + 1) {
            field[i][j] = 0.;
            for i_mode in 0..(size / 2 + 1) {
                for j_mode in 0..size {
                    let arg = (i * i_mode) as f64 + (j * j_mode) as f64;
                    field[i][j] += a_mods[i_mode][j_mode] * (h * arg).cos()
                        + b_mods[i_mode][j_mode] * (h * arg).sin();
                }
            }
        }
    }
}

pub fn second_realization_d_2d_x(
    field: &mut Vec<Vec<f64>>,
    a_mods: &Vec<Vec<f64>>,
    b_mods: &Vec<Vec<f64>>,
) {
    let size = field.len() - 1;

    size_assert_2d(&field, &a_mods, &b_mods);
    nyquist_assert_2d(&field, &a_mods, &b_mods);

    let h = (2. * PI) / (size as f64);

    for i in 0..(size + 1) {
        for j in 0..(size + 1) {
            field[i][j] = 0.;
            for i_mode in 0..(size / 2 + 1) {
                for j_mode in 0..size {
                    let arg = (i * i_mode) as f64 + (j * j_mode) as f64;
                    field[i][j] += -a_mods[i_mode][j_mode] * (h * arg).sin() * i_mode as f64
                        + b_mods[i_mode][j_mode] * (h * arg).cos() * i_mode as f64;
                }
            }
        }
    }
}

pub fn second_realization_d_2d_y(
    field: &mut Vec<Vec<f64>>,
    a_mods: &Vec<Vec<f64>>,
    b_mods: &Vec<Vec<f64>>,
) {
    let size = field.len() - 1;

    size_assert_2d(&field, &a_mods, &b_mods);
    nyquist_assert_2d(&field, &a_mods, &b_mods);

    let h = (2. * PI) / (size as f64);

    for i in 0..(size + 1) {
        for j in 0..(size + 1) {
            field[i][j] = 0.;
            for i_mode in 0..(size / 2 + 1) {
                for j_mode in 0..size {
                    let arg = (i * i_mode) as f64 + (j * j_mode) as f64;
                    field[i][j] += -a_mods[i_mode][j_mode] * (h * arg).sin()
                        * j_mode_normalized(size, j_mode) as f64
                        + b_mods[i_mode][j_mode] * (h * arg).cos()
                            * j_mode_normalized(size, j_mode) as f64;
                }
            }
        }
    }
}

pub fn back(field: &Vec<Vec<f64>>, a_mods: &mut Vec<Vec<f64>>, b_mods: &mut Vec<Vec<f64>>) {
    let size = field.len() - 1;

    size_assert_2d(&field, &a_mods, &b_mods);

    let mut temporary_a: Vec<Vec<f64>> = vec![vec![0.; size / 2 + 1]; size];
    let mut temporary_b: Vec<Vec<f64>> = vec![vec![0.; size / 2 + 1]; size];

    for j in 0..size {
        let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

        for i in 0..size {
            input[i] = Complex::new(field[i][j], 0.);
        }

        let fft = Radix4::new(size, true);
        fft.process(&mut input, &mut output);

        temporary_a[j][0] = output[0].re / (size as f64);
        temporary_a[j][size / 2] = output[size / 2].re / (size as f64);
        for i_mode in 1..(size / 2) {
            temporary_a[j][i_mode] = output[i_mode].re / (size as f64) * 2.;
            temporary_b[j][i_mode] = output[i_mode].im / (size as f64) * 2.;
        }
    }

    for i_mode in 1..(size / 2) {
        let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

        for j in 0..size {
            input[j] = Complex::new(temporary_a[j][i_mode], temporary_b[j][i_mode]);
        }

        let fft = Radix4::new(size, true);
        fft.process(&mut input, &mut output);

        for j_mode in 0..size {
            a_mods[i_mode][j_mode] = output[j_mode].re / (size as f64);
            b_mods[i_mode][j_mode] = output[j_mode].im / (size as f64);
        }
    }

    for i_mode in 0..1 {
        let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

        for j in 0..size {
            input[j] = Complex::new(temporary_a[j][i_mode], 0.);
        }

        let fft = Radix4::new(size, true);
        fft.process(&mut input, &mut output);

        a_mods[i_mode][0] = output[0].re / (size as f64);
        a_mods[i_mode][size / 2] = output[size / 2].re / (size as f64);
        for j_mode in 1..(size / 2) {
            a_mods[i_mode][j_mode] = output[j_mode].re / (size as f64) * 2.;
            b_mods[i_mode][j_mode] = output[j_mode].im / (size as f64) * 2.;
        }
        for j_mode in (size / 2 + 1)..size {
            a_mods[i_mode][j_mode] = 0.;
        }
        for j_mode in (size / 2)..size {
            b_mods[i_mode][j_mode] = 0.;
        }
        b_mods[i_mode][0] = 0.;
    }

    for i_mode in (size / 2)..(size / 2 + 1) {
        let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

        for j in 0..size {
            input[j] = Complex::new(temporary_a[j][i_mode], 0.);
        }

        let fft = Radix4::new(size, true);
        fft.process(&mut input, &mut output);

        a_mods[i_mode][0] = output[0].re / (size as f64);
        a_mods[i_mode][size / 2] = output[size / 2].re / (size as f64);
        for j_mode in 1..(size / 2) {
            a_mods[i_mode][j_mode] = output[j_mode].re / (size as f64) * 2.;
            b_mods[i_mode][j_mode] = output[j_mode].im / (size as f64) * 2.;
        }
        for j_mode in (size / 2 + 1)..size {
            a_mods[i_mode][j_mode] = 0.;
        }
        for j_mode in (size / 2)..size {
            b_mods[i_mode][j_mode] = 0.;
        }
        b_mods[i_mode][0] = 0.;
    }
}
