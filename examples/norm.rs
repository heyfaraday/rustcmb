extern crate rustcmb;

use rustcmb::io::write_1d;

const SIZE: usize = 64;
const DATA_OUT: &str = "data/examples/out/norm/";
const OUTPUT_SIZE: usize = SIZE / 2;

fn main() {

    let mut mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    let max_k_distance = SIZE as f64 / 2.;
    let h = max_k_distance / (OUTPUT_SIZE as f64);

    let mut norm1 = vec![0.; OUTPUT_SIZE + 1];
    let mut norm2 = vec![0.; OUTPUT_SIZE + 1];

    for k1 in 0..1 {
        for k2 in 0..(SIZE / 2 + 1) {
            mods[k1][k2] = 1.;
        }
    }

    for k1 in 1..(SIZE / 2) {
        for k2 in 0..SIZE {
            mods[k1][k2] = 1.;
        }
    }

    for k1 in (SIZE / 2)..(SIZE / 2 + 1) {
        for k2 in 0..(SIZE / 2 + 1) {
            mods[k1][k2] = 1.;
        }
    }

    // Printing
    for k1 in 0..(SIZE / 2 + 1) {
        for k2 in 0..SIZE {
            let k2_f64: f64 = if k2 > (SIZE / 2) {
                k2 as f64 - SIZE as f64
            } else {
                k2 as f64
            };
            let k1_f64: f64 = k1 as f64;
            let k2_distance: f64 = (k1_f64 * k1_f64 + k2_f64 * k2_f64).sqrt();

            if k2_distance > max_k_distance {
                mods[k1][k2] = 0.;
            }

            //println!("k1: {}, k2: {}, mods[k1][k2]: {}", k1_f64, k2_f64, mods[k1][k2]);
        }
    }

    for k1 in 0..(SIZE / 2 + 1) {
        for k2 in 0..SIZE {
            let k2_f64: f64 = if k2 > (SIZE / 2) {
                k2 as f64 - SIZE as f64
            } else {
                k2 as f64
            };
            let k1_f64: f64 = k1 as f64;
            let k2_distance: f64 = (k1_f64 * k1_f64 + k2_f64 * k2_f64).sqrt();

            let index1 = (k2_distance / h).trunc() as usize;

            let index2 = if (k2_distance.fract() / h) == 0. && index1 != 0 {
                index1 - 1
            } else {
                index1
            };

            if k2_distance <= max_k_distance {
                norm1[index1] += mods[k1][k2];
                norm2[index2] += mods[k1][k2];
            }

            //println!{"debug print: k1: {}, k2: {}, index1: {}, index2: {}",
            //k1_f64, k2_f64, index1, index2};
        }
    }

    norm1[OUTPUT_SIZE - 1] += norm1[OUTPUT_SIZE];
    norm1[OUTPUT_SIZE] = 0.;

    println!("norm1: {:?}", norm1);
    println!("norm2: {:?}", norm2);

    println!("Step: {}", h);

    println!("trunc_test: {}", (5.9 / h).trunc() as usize);
    println!("fract_test: {}", (5.9 as f64).fract());

    write_1d(&norm1, DATA_OUT, "norm1.dat");
    write_1d(&norm2, DATA_OUT, "norm2.dat");

    let mut norm1_sum: f64 = 0.;
    let mut norm2_sum: f64 = 0.;
    for i in 0..(OUTPUT_SIZE + 1) {
        norm1_sum += norm1[i];
        norm2_sum += norm2[i];
    }

    println!("norm1_sum: {}, norm2_sum: {}", norm1_sum, norm2_sum);
}
