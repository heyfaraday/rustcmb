extern crate rustfft;

use err::{nyquist_assert_2d, size_assert_2d};
use math::j_mode_normalized;

use self::rustfft::algorithm::Radix4;
use self::rustfft::FFT;
use self::rustfft::num_complex::Complex;
use self::rustfft::num_traits::Zero;

pub fn d_2d_x(field: &mut Vec<Vec<f64>>, a_mods: &Vec<Vec<f64>>, b_mods: &Vec<Vec<f64>>) {
    let size = field.len() - 1;

    size_assert_2d(&field, &a_mods, &b_mods);
    nyquist_assert_2d(&field, &a_mods, &b_mods);

    let mut temporary: Vec<Vec<Complex<f64>>> = vec![vec![Complex::zero(); size]; size / 2 + 1];

    for i_mode in 0..(size / 2 + 1) {
        let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

        for j_mode in 0..size {
            input[j_mode] = Complex::new(
                i_mode as f64 * b_mods[i_mode][j_mode],
                -(i_mode as f64) * a_mods[i_mode][j_mode],
            );
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

pub fn d_2d_y(field: &mut Vec<Vec<f64>>, a_mods: &Vec<Vec<f64>>, b_mods: &Vec<Vec<f64>>) {
    let size = field.len() - 1;

    size_assert_2d(&field, &a_mods, &b_mods);
    nyquist_assert_2d(&field, &a_mods, &b_mods);

    let mut temporary: Vec<Vec<Complex<f64>>> = vec![vec![Complex::zero(); size]; size / 2 + 1];

    for i_mode in 0..(size / 2 + 1) {
        let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

        for j_mode in 0..size {
            input[j_mode] = Complex::new(
                j_mode as f64 * b_mods[i_mode][j_mode],
                -(j_mode as f64) * a_mods[i_mode][j_mode],
            );
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

pub fn d_2d_xx(field: &mut Vec<Vec<f64>>, a_mods: &Vec<Vec<f64>>, b_mods: &Vec<Vec<f64>>) {
    let size = field.len() - 1;

    size_assert_2d(&field, &a_mods, &b_mods);
    nyquist_assert_2d(&field, &a_mods, &b_mods);

    let mut temporary: Vec<Vec<Complex<f64>>> = vec![vec![Complex::zero(); size]; size / 2 + 1];

    for i_mode in 0..(size / 2 + 1) {
        let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

        for j_mode in 0..size {
            input[j_mode] = Complex::new(
                -(i_mode as f64) * (i_mode as f64) * a_mods[i_mode][j_mode],
                -(i_mode as f64) * (i_mode as f64) * b_mods[i_mode][j_mode],
            );
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

pub fn d_2d_yy(field: &mut Vec<Vec<f64>>, a_mods: &Vec<Vec<f64>>, b_mods: &Vec<Vec<f64>>) {
    let size = field.len() - 1;

    size_assert_2d(&field, &a_mods, &b_mods);
    nyquist_assert_2d(&field, &a_mods, &b_mods);

    let mut temporary: Vec<Vec<Complex<f64>>> = vec![vec![Complex::zero(); size]; size / 2 + 1];

    for i_mode in 0..(size / 2 + 1) {
        let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

        for j_mode in 0..size {
            input[j_mode] = Complex::new(
                -(j_mode as f64) * (j_mode as f64) * a_mods[i_mode][j_mode],
                -(j_mode as f64) * (j_mode as f64) * b_mods[i_mode][j_mode],
            );
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

pub fn d_2d_xy(field: &mut Vec<Vec<f64>>, a_mods: &Vec<Vec<f64>>, b_mods: &Vec<Vec<f64>>) {
    let size = field.len() - 1;

    size_assert_2d(&field, &a_mods, &b_mods);
    nyquist_assert_2d(&field, &a_mods, &b_mods);

    let mut temporary: Vec<Vec<Complex<f64>>> = vec![vec![Complex::zero(); size]; size / 2 + 1];

    for i_mode in 0..(size / 2 + 1) {
        let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

        for j_mode in 0..size {
            input[j_mode] = Complex::new(
                -(i_mode as f64) * (j_mode as f64) * a_mods[i_mode][j_mode],
                -(i_mode as f64) * (j_mode as f64) * b_mods[i_mode][j_mode],
            );
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

pub fn d_x(a_mods: &Vec<Vec<f64>>, b_mods: &Vec<Vec<f64>>) -> [Vec<Vec<f64>>; 2] {
    let size = a_mods[0].len();

    let mut a_new: Vec<Vec<f64>> = vec![vec![0.; size]; size / 2 + 1];
    let mut b_new: Vec<Vec<f64>> = vec![vec![0.; size]; size / 2 + 1];

    for i_mode in 1..(size / 2) {
        for j_mode in 0..size {
            a_new[i_mode][j_mode] = b_mods[i_mode][j_mode] * (i_mode as f64);
            b_new[i_mode][j_mode] = a_mods[i_mode][j_mode] * (i_mode as f64 * -1.);
        }
    }
    for i_mode in (size / 2)..(size / 2 + 1) {
        for j_mode in 1..(size / 2) {
            a_new[i_mode][j_mode] = b_mods[i_mode][j_mode] * (i_mode as f64);
            b_new[i_mode][j_mode] = a_mods[i_mode][j_mode] * (i_mode as f64 * -1.);
        }
    }
    [a_new, b_new]
}

pub fn d_y(a_mods: &Vec<Vec<f64>>, b_mods: &Vec<Vec<f64>>) -> [Vec<Vec<f64>>; 2] {
    let size = a_mods[0].len();

    let mut a_new: Vec<Vec<f64>> = vec![vec![0.; size]; size / 2 + 1];
    let mut b_new: Vec<Vec<f64>> = vec![vec![0.; size]; size / 2 + 1];

    for i_mode in 0..1 {
        for j_mode in 1..(size / 2) {
            a_new[i_mode][j_mode] =
                b_mods[i_mode][j_mode] * (j_mode_normalized(size, j_mode) as f64);
            b_new[i_mode][j_mode] =
                a_mods[i_mode][j_mode] * (-j_mode_normalized(size, j_mode) as f64);
        }
    }

    for i_mode in 1..(size / 2) {
        for j_mode in 1..size {
            a_new[i_mode][j_mode] =
                b_mods[i_mode][j_mode] * (j_mode_normalized(size, j_mode) as f64);
            b_new[i_mode][j_mode] =
                a_mods[i_mode][j_mode] * (-j_mode_normalized(size, j_mode) as f64);
        }
    }

    for i_mode in (size / 2)..(size / 2 + 1) {
        for j_mode in 1..(size / 2) {
            a_new[i_mode][j_mode] =
                b_mods[i_mode][j_mode] * (j_mode_normalized(size, j_mode) as f64);
            b_new[i_mode][j_mode] =
                a_mods[i_mode][j_mode] * (-j_mode_normalized(size, j_mode) as f64);
        }
    }
    [a_new, b_new]
}
