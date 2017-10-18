extern crate rustcmb;

use rustcmb::fourier::fft_2d;
use rustcmb::io::write_2d;
use rustcmb::spectra::gasdev_exp_k0;

const DATA_OUT: &str = "data/examples/out/corr_2d/";
const SIZE: usize = 64;

fn main() {

    let mut field: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    gasdev_exp_k0(&field, &mut a_mods, &mut b_mods, 0., 1., 10.);

    fft_2d::torus::second_realization(&mut field, &a_mods, &b_mods);
    write_2d(&field, &DATA_OUT, &"field.dat");

    let delta_i = -2 as i32;
    let delta_j = -1 as i32;

    for i in 0..SIZE {
        for j in 0..SIZE {

            let mut mod_delta_i = i as i32 + delta_i;
            let mut mod_delta_j = j as i32 + delta_j;

            if mod_delta_i >= SIZE as i32 {
                mod_delta_i -= SIZE as i32;
            } else if mod_delta_i < 0 {
                mod_delta_i += SIZE as i32;
            }

            if mod_delta_j >= SIZE as i32 {
                mod_delta_j -= SIZE as i32;
            } else if mod_delta_j < 0 {
                mod_delta_j += SIZE as i32;
            }

            println!(
                "i1: {}, j1: {}, i2: {}, j2: {}",
                i,
                j,
                mod_delta_i,
                mod_delta_j
            );
        }
    }

    println!("\n");

    let mut cor_func: Vec<Vec<f64>> = vec![vec![0.; 2 * SIZE - 1]; 2 * SIZE - 1];

    for delta_i in (-(SIZE as i32) + 1)..SIZE as i32 {
        for delta_j in (-(SIZE as i32) + 1)..SIZE as i32 {

            let mut sum = 0.;

            for i in 0..SIZE {
                for j in 0..SIZE {

                    let mut mod_delta_i = i as i32 + delta_i;
                    let mut mod_delta_j = j as i32 + delta_j;

                    if mod_delta_i >= SIZE as i32 {
                        mod_delta_i -= SIZE as i32;
                    } else if mod_delta_i < 0 {
                        mod_delta_i += SIZE as i32;
                    }

                    if mod_delta_j >= SIZE as i32 {
                        mod_delta_j -= SIZE as i32;
                    } else if mod_delta_j < 0 {
                        mod_delta_j += SIZE as i32;
                    }

                    sum += field[i][j] * field[mod_delta_i as usize][mod_delta_j as usize];
                }
            }

            cor_func[(delta_i + SIZE as i32 - 1) as usize][(delta_j + SIZE as i32 - 1) as usize] =
                sum;

            println!(
                "delta_i: {}, delta_j: {}, cor_func: {}",
                delta_i,
                delta_j,
                cor_func[(delta_i + SIZE as i32 - 1) as usize][(delta_j + SIZE as i32 - 1) as usize]
            );
        }
    }

    write_2d(&cor_func, &DATA_OUT, &"cor_func.dat");
}
