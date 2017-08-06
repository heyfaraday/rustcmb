extern crate rand;

use self::rand::distributions::normal::StandardNormal;

pub fn gasdev(a: &mut Vec<Vec<f64>>, b: &mut Vec<Vec<f64>>, mean: f64, std: f64) {

    let size = a.capacity() - 1;

    for i in 0..(size / 2 + 1) {
        for j in 0..(size / 2 + 1) {

            let StandardNormal(x) = rand::random();
            let mode: f64 = x * std + mean;

            a[i][j] = mode;
        }
    }

    for i in 1..(size / 2) {
        for j in 1..(size / 2) {

            let StandardNormal(x) = rand::random();
            let mode: f64 = x * std + mean;

            b[i][j] = mode;
        }
    }
}
