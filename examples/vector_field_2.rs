extern crate rustcmb;

use rustcmb::io::{write_1d, write_2d};
use rustcmb::spectra::gasdev_max_k;
use rustcmb::fourier::fft_2d::torus::first_realization;
//use rustcmb::fourier::fft_2d::torus::second_realization;

use rustcmb::corr::{two_point_correlation_function_with_all_distances,
                    two_point_correlation_function_2d, two_point_correlation_function_2d_dummy};

use rustcmb::corr::correlation_2d_to_1d_with_all_distances;

const SIZE: usize = 64;
const DATA_OUT: &str = "../data/out/rust-examples/vector_field_2/";

fn main() {
    let mut field: Vec<Vec<f64>> = vec![vec![0.; SIZE + 1]; SIZE + 1];
    let mut a_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];
    let mut b_mods: Vec<Vec<f64>> = vec![vec![0.; SIZE]; SIZE / 2 + 1];

    gasdev_max_k(&field, &mut a_mods, &mut b_mods, 0., 1., 5.);
    //    a_mods[0][1] = 1.;

    first_realization(&mut field, &a_mods, &b_mods);
    //    second_realization(&mut field, &a_mods, &b_mods);

    let correlation = two_point_correlation_function_with_all_distances(&field, &field);

    {
        write_2d(&field, &DATA_OUT, &"field.dat");
        write_2d(&a_mods, &DATA_OUT, &"a_mods.dat");
        write_2d(&b_mods, &DATA_OUT, &"b_mods.dat");

        write_1d(&correlation[0], &DATA_OUT, &"output.dat");
        write_1d(&correlation[1], &DATA_OUT, &"norm_for_output.dat");
        write_1d(&correlation[2], &DATA_OUT, &"distances.dat");
        write_1d(&correlation[3], &DATA_OUT, &"initial_norm_for_output.dat");
    }

    let correlation_2d_dummy =
        two_point_correlation_function_2d_dummy(&field, &field, |x, y| x * y);
    {
        write_2d(
            &correlation_2d_dummy[0],
            &DATA_OUT,
            &"correlation_dummy.dat",
        );
        write_2d(
            &correlation_2d_dummy[1],
            &DATA_OUT,
            &"norm_for_correlation_dummy.dat",
        );
    }

    let correlation_2d = two_point_correlation_function_2d(&field, &field, |x, y| x * y);
    {
        write_2d(&correlation_2d[0], &DATA_OUT, &"correlation.dat");
        write_2d(&correlation_2d[1], &DATA_OUT, &"norm_for_correlation.dat");
    }

    let correlation_1d = correlation_2d_to_1d_with_all_distances(&correlation_2d[0]);

    {
        write_1d(&correlation_1d[0], &DATA_OUT, &"output_from_2d.dat");
        write_1d(
            &correlation_1d[1],
            &DATA_OUT,
            &"norm_for_output_from_2d.dat",
        );
        write_1d(&correlation_1d[2], &DATA_OUT, &"distances_from_2d.dat");
        write_1d(
            &correlation_1d[3],
            &DATA_OUT,
            &"initial_norm_for_output_from_2d.dat",
        );
    }
}
