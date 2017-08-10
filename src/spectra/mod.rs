extern crate rand;

use err::size_assert_2d;
use math::{fourier_distance, normal_generator, normal_generator_max};

use self::rand::distributions::normal::StandardNormal;

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

            let StandardNormal(x) = rand::random();
            let mode: f64 = x * std + mean;

            a[i_mode][j_mode] = mode;
        }

        for j_mode in (size / 2 + 1)..size {
            a[i_mode][j_mode] = 0.;
        }

        for j_mode in 1..(size / 2) {

            let StandardNormal(x) = rand::random();
            let mode: f64 = x * std + mean;

            b[i_mode][j_mode] = mode;
        }
        for j_mode in (size / 2)..size {
            b[i_mode][j_mode] = 0.;
        }
    }

    for i_mode in 1..(size / 2) {
        for j_mode in 0..size {

            let StandardNormal(x) = rand::random();
            let mode: f64 = x * std + mean;

            a[i_mode][j_mode] = mode;

            let StandardNormal(x) = rand::random();
            let mode: f64 = x * std + mean;

            b[i_mode][j_mode] = mode;
        }
    }

    for i_mode in (size / 2)..(size / 2 + 1) {
        for j_mode in 0..(size / 2 + 1) {

            let StandardNormal(x) = rand::random();
            let mode: f64 = x * std + mean;

            a[i_mode][j_mode] = mode;
        }

        for j_mode in (size / 2 + 1)..size {
            a[i_mode][j_mode] = 0.;
        }

        for j_mode in 1..(size / 2) {

            let StandardNormal(x) = rand::random();
            let mode: f64 = x * std + mean;

            b[i_mode][j_mode] = mode;
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
            a[i_mode][j_mode] = normal_generator(mean, std0 * (-(arg * arg) / (k0 * k0)).exp());
        }

        for j_mode in (size / 2 + 1)..size {
            a[i_mode][j_mode] = 0.;
        }

        for j_mode in 1..(size / 2) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            b[i_mode][j_mode] = normal_generator(mean, std0 * (-(arg * arg) / (k0 * k0)).exp());
        }
        for j_mode in (size / 2)..size {
            b[i_mode][j_mode] = 0.;
        }
    }

    for i_mode in 1..(size / 2) {
        for j_mode in 0..size {

            let arg = fourier_distance(&field, i_mode, j_mode);
            a[i_mode][j_mode] = normal_generator(mean, std0 * (-(arg * arg) / (k0 * k0)).exp());

            let arg = fourier_distance(&field, i_mode, j_mode);
            b[i_mode][j_mode] = normal_generator(mean, std0 * (-(arg * arg) / (k0 * k0)).exp());
        }
    }

    for i_mode in (size / 2)..(size / 2 + 1) {
        for j_mode in 0..(size / 2 + 1) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            a[i_mode][j_mode] = normal_generator(mean, std0 * (-(arg * arg) / (k0 * k0)).exp());
        }

        for j_mode in (size / 2 + 1)..size {
            a[i_mode][j_mode] = 0.;
        }

        for j_mode in 1..(size / 2) {

            let arg = fourier_distance(&field, i_mode, j_mode);
            b[i_mode][j_mode] = normal_generator(mean, std0 * (-(arg * arg) / (k0 * k0)).exp());
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
