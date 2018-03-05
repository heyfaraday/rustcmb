extern crate rand;

use std::fs::File;
use std::io::prelude::Write;
use rand::distributions::normal::StandardNormal;
use std::f64::consts::PI;

const NUMBER_FOR_SAMPLING: usize = 100000;

const XI_XX: f64 = -0.5;
const XI_YY: f64 = 0.7;
const XI_XY: f64 = 0.9;

const XI_XX_0: f64 = -1.;
const XI_YY_0: f64 = -1.;

fn main() {
    let mut file =
        File::create("data/examples/out/mcmc/sample.dat").expect("Unable to create file");

    for _ in 0..NUMBER_FOR_SAMPLING {
        let f_plus = XI_XY / ((1. + XI_XX) * (1. + XI_YY)).sqrt();
        let f_minus = -XI_XY / ((1. - XI_XX) * (1. - XI_XX)).sqrt();

        let sigma: [f64; 4] = [
            2. * (1. + f_plus),
            2. * (1. - f_plus),
            2. * (1. + f_minus),
            2. * (1. - f_minus),
        ];

        let delta_1: f64 = random_normal_number() * sigma[0].sqrt();
        let delta_2: f64 = random_normal_number() * sigma[1].sqrt();
        let delta_3: f64 = random_normal_number() * sigma[2].sqrt();
        let delta_4: f64 = random_normal_number() * sigma[3].sqrt();

        let wx_plus = 0.5 * (delta_1 + delta_2) * (2. * (1. + XI_XX)).sqrt();
        let wy_plus = 0.5 * (delta_1 - delta_2) * (2. * (1. + XI_YY)).sqrt();
        let wx_minus = 0.5 * (delta_3 + delta_4) * (2. * (1. - XI_XX)).sqrt();
        let wy_minus = 0.5 * (delta_3 - delta_4) * (2. * (1. - XI_YY)).sqrt();

        let wx1 = 0.5 * (wx_plus + wx_minus);
        let wx2 = 0.5 * (wx_plus - wx_minus);
        let wy1 = 0.5 * (wy_plus + wy_minus);
        let wy2 = 0.5 * (wy_plus - wy_minus);

        let vx1 = wx1 / (-XI_XX_0).sqrt();
        let vx2 = wx2 / (-XI_XX_0).sqrt();
        let vy1 = wy1 / (-XI_YY_0).sqrt();
        let vy2 = wy2 / (-XI_YY_0).sqrt();

        let v1 = (vx1 * vx1 + vy1 * vy1).sqrt();
        let v2 = (vx2 * vx2 + vy2 * vy2).sqrt();

        let cos1 = vx1 / v1;
        let sin1 = vy1 / v1;
        let cos2 = vx2 / v2;
        let sin2 = vy2 / v2;

        let phi1: f64;
        let phi2: f64;

        if sin1 > 0. {
            phi1 = cos1.acos();
        } else {
            phi1 = 2. * PI - cos1.acos();
        }

        if sin2 > 0. {
            phi2 = cos2.acos();
        } else {
            phi2 = 2. * PI - cos2.acos();
        }

        file.write_all((format!("{}\t{}\t{}\t{}\n", phi1, phi2, v1, v2)).as_bytes())
            .expect("Unable to write data");
    }
}

#[inline]
fn random_normal_number() -> f64 {
    let StandardNormal(x) = rand::random();
    x
}
