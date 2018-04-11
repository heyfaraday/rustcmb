extern crate rand;

use err::size_assert_2d_for_field;

use self::rand::distributions::normal::StandardNormal;

use err::point_assert;

use std::f64::consts::PI;

pub fn torus_distance(field: &Vec<Vec<f64>>, point_1: &[usize; 2], point_2: &[usize; 2]) -> f64 {
    let size = field.len() - 1;

    point_assert(&field, &point_1);
    point_assert(&field, &point_2);

    let i: f64 = if point_1[0] >= point_2[0] {
        point_1[0] as f64 - point_2[0] as f64
    } else {
        point_2[0] as f64 - point_1[0] as f64
    };
    let j: f64 = if point_1[1] >= point_2[1] {
        point_1[1] as f64 - point_2[1] as f64
    } else {
        point_2[1] as f64 - point_1[1] as f64
    };

    let i: f64 = if i > (size as f64) / 2. {
        size as f64 - i
    } else {
        i
    };
    let j: f64 = if j > (size as f64) / 2. {
        size as f64 - j
    } else {
        j
    };

    let arg: f64 = i * i + j * j;

    arg.sqrt()
}

pub fn torus_distance_square(
    field: &Vec<Vec<f64>>,
    point_1: &[usize; 2],
    point_2: &[usize; 2],
) -> f64 {
    let size = field.len() - 1;

    point_assert(&field, &point_1);
    point_assert(&field, &point_2);

    let i: f64 = if point_1[0] >= point_2[0] {
        point_1[0] as f64 - point_2[0] as f64
    } else {
        point_2[0] as f64 - point_1[0] as f64
    };
    let j: f64 = if point_1[1] >= point_2[1] {
        point_1[1] as f64 - point_2[1] as f64
    } else {
        point_2[1] as f64 - point_1[1] as f64
    };

    let i: f64 = if i > (size as f64) / 2. {
        size as f64 - i
    } else {
        i
    };
    let j: f64 = if j > (size as f64) / 2. {
        size as f64 - j
    } else {
        j
    };

    let arg: f64 = i * i + j * j;

    arg
}

pub fn angle_for_vector_field(vx: &f64, vy: &f64) -> f64 {
    assert!(*vx != 0. && *vy != 0., "vx and vy equal zero");

    if *vx == 0. {
        if *vy > 0. {
            PI / 2.
        } else {
            -PI / 2.
        }
    } else {
        let tg: f64 = vy / vx;
        let mut phi = tg.atan();

        if *vx < 0. {
            if *vy > 0. {
                phi += PI
            } else {
                phi -= PI
            }
        }
        phi
    }
}

pub fn angle_for_tensor_field(q: &f64, u: &f64) -> f64 {
    assert!(*q != 0. && *u != 0., "q and u equal zero");

    if *q == 0. {
        if *u > 0. {
            PI / 4.
        } else {
            -PI / 4.
        }
    } else {
        let tg: f64 = u / q;
        let mut double_phi = tg.atan();

        if *q < 0. {
            if *u > 0. {
                double_phi += PI
            } else {
                double_phi -= PI
            }
        }
        double_phi / 2.
    }
}

pub fn j_mode_normalized(size: usize, j: usize) -> i32 {
    if j <= size / 2 {
        j as i32
    } else {
        j as i32 - size as i32
    }
}

pub fn fourier_distance(field: &Vec<Vec<f64>>, k1: usize, k2: usize) -> f64 {
    let size = field.len() - 1;

    size_assert_2d_for_field(&field);
    assert!(k1 < (size / 2 + 1));
    assert!(k2 < size);

    let d1 = k1 as f64;
    let d2 = if k2 as f64 <= size as f64 / 2. {
        k2 as f64
    } else {
        k2 as f64 - size as f64
    };

    (d1 * d1 + d2 * d2).sqrt()
}

#[inline]
pub fn normal_generator(mean: f64, std: f64) -> f64 {
    let StandardNormal(x) = rand::random();
    mean + x * std
}

#[inline]
pub fn normal_generator_max(mean: f64, std: f64, arg: f64, max_arg: f64) -> f64 {
    if arg > max_arg {
        mean
    } else {
        normal_generator(mean, std)
    }
}
