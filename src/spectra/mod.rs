extern crate rand;

use err::size_assert_2d;
use math::{fourier_distance, normal_generator, normal_generator_max};

pub fn gasdev(
    field: &Vec<Vec<f64>>,
    a: &mut Vec<Vec<f64>>,
    b: &mut Vec<Vec<f64>>,
    mean: f64,
    std: f64,
) {

    let size = field.capacity() - 1;

    size_assert_2d(&field, &a, &b);

    for i_mode in 0..1 {
        for j_mode in 0..(size / 2 + 1) {

            a[i_mode][j_mode] = normal_generator(mean, std);
        }

        for j_mode in (size / 2 + 1)..size {
            a[i_mode][j_mode] = 0.;
        }

        for j_mode in 1..(size / 2) {

            b[i_mode][j_mode] = normal_generator(mean, std);
        }
        for j_mode in (size / 2)..size {
            b[i_mode][j_mode] = 0.;
        }
    }

    for i_mode in 1..(size / 2) {
        for j_mode in 0..size {

            a[i_mode][j_mode] = normal_generator(mean, std);

            b[i_mode][j_mode] = normal_generator(mean, std);
        }
    }

    for i_mode in (size / 2)..(size / 2 + 1) {
        for j_mode in 0..(size / 2 + 1) {

            a[i_mode][j_mode] = normal_generator(mean, std);
        }

        for j_mode in (size / 2 + 1)..size {
            a[i_mode][j_mode] = 0.;
        }

        for j_mode in 1..(size / 2) {

            b[i_mode][j_mode] = normal_generator(mean, std);
        }
        for j_mode in (size / 2)..size {
            b[i_mode][j_mode] = 0.;
        }
    }
}

pub fn gasdev_exp_k0(
    field: &Vec<Vec<f64>>,
    a: &mut Vec<Vec<f64>>,
    b: &mut Vec<Vec<f64>>,
    mean: f64,
    std0: f64,
    k0: f64,
) {

    let size = field.capacity() - 1;
    size_assert_2d(&field, &a, &b);
    assert!(k0 != 0.);

    for i_mode in 0..1 {
        for j_mode in 0..(size / 2 + 1) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            a[i_mode][j_mode] = normal_generator(mean, std0 * exp(&arg, &k0, &1.));
        }

        for j_mode in (size / 2 + 1)..size {
            a[i_mode][j_mode] = 0.;
        }

        for j_mode in 1..(size / 2) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            b[i_mode][j_mode] = normal_generator(mean, std0 * exp(&arg, &k0, &1.));
        }

        for j_mode in (size / 2)..size {
            b[i_mode][j_mode] = 0.;
        }
    }

    for i_mode in 1..(size / 2) {
        for j_mode in 0..size {

            let arg = fourier_distance(&field, i_mode, j_mode);
            a[i_mode][j_mode] = normal_generator(mean, std0 * exp(&arg, &k0, &1.));
            b[i_mode][j_mode] = normal_generator(mean, std0 * exp(&arg, &k0, &1.));
        }
    }

    for i_mode in (size / 2)..(size / 2 + 1) {
        for j_mode in 0..(size / 2 + 1) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            a[i_mode][j_mode] = normal_generator(mean, std0 * exp(&arg, &k0, &1.));
        }

        for j_mode in (size / 2 + 1)..size {
            a[i_mode][j_mode] = 0.;
        }

        for j_mode in 1..(size / 2) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            b[i_mode][j_mode] = normal_generator(mean, std0 * exp(&arg, &k0, &1.));
        }

        for j_mode in (size / 2)..size {
            b[i_mode][j_mode] = 0.;
        }
    }
}

pub fn gasdev_max_k(
    field: &Vec<Vec<f64>>,
    a: &mut Vec<Vec<f64>>,
    b: &mut Vec<Vec<f64>>,
    mean: f64,
    std0: f64,
    max: f64,
) {

    let size = field.capacity() - 1;

    size_assert_2d(&field, &a, &b);

    for i_mode in 0..1 {
        for j_mode in 0..(size / 2 + 1) {
            let arg = fourier_distance(&field, i_mode, j_mode);
            a[i_mode][j_mode] = normal_generator_max(mean, std0, arg, max);
        }

        for j_mode in (size / 2 + 1)..size {
            a[i_mode][j_mode] = 0.;
        }

        for j_mode in 1..(size / 2) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            b[i_mode][j_mode] = normal_generator_max(mean, std0, arg, max);
        }
        for j_mode in (size / 2)..size {
            b[i_mode][j_mode] = 0.;
        }
    }

    for i_mode in 1..(size / 2) {
        for j_mode in 0..size {

            let arg = fourier_distance(&field, i_mode, j_mode);
            a[i_mode][j_mode] = normal_generator_max(mean, std0, arg, max);

            let arg = fourier_distance(&field, i_mode, j_mode);
            b[i_mode][j_mode] = normal_generator_max(mean, std0, arg, max);
        }
    }

    for i_mode in (size / 2)..(size / 2 + 1) {
        for j_mode in 0..(size / 2 + 1) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            a[i_mode][j_mode] = normal_generator_max(mean, std0, arg, max);
        }

        for j_mode in (size / 2 + 1)..size {
            a[i_mode][j_mode] = 0.;
        }

        for j_mode in 1..(size / 2) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            b[i_mode][j_mode] = normal_generator_max(mean, std0, arg, max);
        }
        for j_mode in (size / 2)..size {
            b[i_mode][j_mode] = 0.;
        }
    }

}

pub fn gasdev_exp_and_sin(
    field: &Vec<Vec<f64>>,
    a: &mut Vec<Vec<f64>>,
    b: &mut Vec<Vec<f64>>,
    mean: f64,
    null_gap: f64,
    exp_gap: f64,
    sin_gap: f64,
    null_param: f64,
    exp_param: f64,
    sin_param: f64,
) {

    let size = field.capacity() - 1;
    size_assert_2d(&field, &a, &b);
    assert!(null_gap >= 0.);

    for i_mode in 0..1 {
        for j_mode in 0..(size / 2 + 1) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            a[i_mode][j_mode] = normal_generator(
                mean,
                exp_and_sin(
                    &size,
                    &arg,
                    &null_gap,
                    &exp_gap,
                    &sin_gap,
                    &null_param,
                    &exp_param,
                    &sin_param,
                ),
            );
        }

        for j_mode in (size / 2 + 1)..size {
            a[i_mode][j_mode] = 0.;
        }

        for j_mode in 1..(size / 2) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            b[i_mode][j_mode] = normal_generator(
                mean,
                exp_and_sin(
                    &size,
                    &arg,
                    &null_gap,
                    &exp_gap,
                    &sin_gap,
                    &null_param,
                    &exp_param,
                    &sin_param,
                ),
            );
        }

        for j_mode in (size / 2)..size {
            b[i_mode][j_mode] = 0.;
        }
    }

    for i_mode in 1..(size / 2) {
        for j_mode in 0..size {

            let arg = fourier_distance(&field, i_mode, j_mode);
            a[i_mode][j_mode] = normal_generator(
                mean,
                exp_and_sin(
                    &size,
                    &arg,
                    &null_gap,
                    &exp_gap,
                    &sin_gap,
                    &null_param,
                    &exp_param,
                    &sin_param,
                ),
            );
            b[i_mode][j_mode] = normal_generator(
                mean,
                exp_and_sin(
                    &size,
                    &arg,
                    &null_gap,
                    &exp_gap,
                    &sin_gap,
                    &null_param,
                    &exp_param,
                    &sin_param,
                ),
            );
        }
    }

    for i_mode in (size / 2)..(size / 2 + 1) {
        for j_mode in 0..(size / 2 + 1) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            a[i_mode][j_mode] = normal_generator(
                mean,
                exp_and_sin(
                    &size,
                    &arg,
                    &null_gap,
                    &exp_gap,
                    &sin_gap,
                    &null_param,
                    &exp_param,
                    &sin_param,
                ),
            );
        }

        for j_mode in (size / 2 + 1)..size {
            a[i_mode][j_mode] = 0.;
        }

        for j_mode in 1..(size / 2) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            b[i_mode][j_mode] = normal_generator(
                mean,
                exp_and_sin(
                    &size,
                    &arg,
                    &null_gap,
                    &exp_gap,
                    &sin_gap,
                    &null_param,
                    &exp_param,
                    &sin_param,
                ),
            );
        }

        for j_mode in (size / 2)..size {
            b[i_mode][j_mode] = 0.;
        }
    }
}

pub fn return_spectra(
    field: &Vec<Vec<f64>>,
    a: &Vec<Vec<f64>>,
    b: &Vec<Vec<f64>>,
    number_of_bins: usize,
) -> Vec<f64> {

    let size = field.capacity() - 1;
    size_assert_2d(&field, &a, &b);

    let mut spectra: Vec<f64> = vec![0.; number_of_bins];
    let mut norm_for_spectra: Vec<f64> = vec![0.; number_of_bins];

    for i_mode in 0..(size / 2 + 1) {
        for j_mode in 0..size {

            let mut index = (fourier_distance(&field, i_mode, j_mode) /
                                 (size as f64 / (2. as f64).sqrt()) *
                                 number_of_bins as f64)
                .trunc() as usize;

            if index == number_of_bins {
                index -= 1;
            }

            println!("i: {}, j: {}, Index: {}\n", i_mode, j_mode, index);

            spectra[index] += a[i_mode][j_mode] * a[i_mode][j_mode] +
                b[i_mode][j_mode] * b[i_mode][j_mode];
            norm_for_spectra[index] += 1.;
        }
    }

    for i in 0..number_of_bins {
        spectra[i] /= norm_for_spectra[i];
    }
    spectra
}

#[inline]
fn exp_and_sin(
    size: &usize,
    arg: &f64,
    null_gap: &f64,
    exp_gap: &f64,
    sin_gap: &f64,
    null_param: &f64,
    exp_param: &f64,
    sin_param: &f64,
) -> f64 {
    if arg < null_gap {
        0.
    } else {
        null_param * (-2. / *size as f64 * (arg - exp_gap) * exp_param).exp() *
            (2. / *size as f64 * (arg - sin_gap) * sin_param).sin()
    }
}

#[inline]
fn exp(arg: &f64, k0: &f64, null_param: &f64) -> f64 {
    null_param * (-arg / k0).exp()
}
