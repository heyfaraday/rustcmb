extern crate rand;

use err::size_assert_2d;

use self::rand::distributions::normal::StandardNormal;

pub fn gasdev(
    field: &Vec<Vec<f64>>,
    a: &mut Vec<Vec<f64>>,
    b: &mut Vec<Vec<f64>>,
    mean: f64,
    std: f64,
) {

    let size: usize = field.capacity() - 1;

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
