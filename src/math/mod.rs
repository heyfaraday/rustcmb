use std::f64::consts::PI;

pub fn torus_distance(field: &Vec<Vec<f64>>, point_1: &[usize; 2], point_2: &[usize; 2]) -> f64 {

    let size: usize = field.capacity() - 1;

    assert!(field.capacity() == size + 1);
    assert!(point_1[0] < size + 1);
    assert!(point_1[1] < size + 1);
    assert!(point_2[0] < size + 1);
    assert!(point_2[1] < size + 1);

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

pub fn angle_for_vector_field(vx: &f64, vy: &f64) -> f64 {

    assert!(*vx != 0.);

    let tg: f64 = vy / vx;
    let mut phi = tg.atan();

    if *vx < 0. {
        phi += PI;
    }

    phi
}
