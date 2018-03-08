use err::{point_assert, size_assert_2d_for_field};
use math::{angle_for_vector_field, torus_distance};

pub fn correlation_function(field: &Vec<Vec<f64>>, output_size: usize) -> [Vec<f64>; 2] {
    let size = field.capacity() - 1;

    size_assert_2d_for_field(&field);

    let max_distance = torus_distance(&field, &[0, 0], &[size / 2, size / 2]);

    let mut output = vec![0.; output_size];
    let mut norm_for_output = vec![0.; output_size];
    let h = max_distance / (output_size as f64);

    let mut index;

    for i_first_point in 0..size {
        for j_first_point in 0..size {
            let point_1 = [i_first_point, j_first_point];

            for j_second_point in (j_first_point + 1)..size {
                let point_2 = [i_first_point, j_second_point];
                index = (torus_distance(&field, &point_1, &point_2) / h).trunc() as usize;
                if index == output_size {
                    index -= 1;
                }
                output[index] +=
                    field[i_first_point][j_first_point] * field[i_first_point][j_second_point];
                norm_for_output[index] += 1.;
            }

            for i_second_point in (i_first_point + 1)..size {
                for j_second_point in 0..size {
                    let point_2 = [i_second_point, j_second_point];
                    index = (torus_distance(&field, &point_1, &point_2) / h).trunc() as usize;
                    if index == output_size {
                        index -= 1;
                    }
                    output[index] +=
                        field[i_first_point][j_first_point] * field[i_second_point][j_second_point];
                    norm_for_output[index] += 1.;
                }
            }
        }
    }
    for i in 0..output_size {
        output[i] = output[i] / norm_for_output[i];
    }
    [output, norm_for_output]
}

pub fn correlation_check(field: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let size = field.capacity() - 1;

    size_assert_2d_for_field(&field);

    let mut check = vec![vec![0.; size + 1]; size + 1];

    for i_first_point in 0..size {
        for j_first_point in 0..size {
            for j_second_point in (j_first_point + 1)..size {
                check[i_first_point][j_first_point] += 1.;
                check[i_first_point][j_second_point] += 1.;
            }

            for i_second_point in (i_first_point + 1)..size {
                for j_second_point in 0..size {
                    check[i_first_point][j_first_point] += 1.;
                    check[i_second_point][j_second_point] += 1.;
                }
            }
        }
    }
    check
}

pub fn correlation_distance(field: &Vec<Vec<f64>>, point: &[usize; 2]) -> Vec<Vec<f64>> {
    let size = field.capacity() - 1;

    point_assert(&field, &point);
    size_assert_2d_for_field(&field);

    let mut distance = vec![vec![0.; size + 1]; size + 1];

    for i_first_point in 0..size {
        for j_first_point in 0..size {
            distance[i_first_point][j_first_point] =
                torus_distance(&field, &point, &[i_first_point, j_first_point]);
        }
    }
    distance
}

pub fn correlation_function_vector_field(
    vx: &Vec<Vec<f64>>,
    vy: &Vec<Vec<f64>>,
    output_size: usize,
) -> [Vec<f64>; 3] {
    let size = vx.capacity() - 1;
    let size_2 = vy.capacity() - 1;
    assert!(size == size_2);

    size_assert_2d_for_field(&vx);
    size_assert_2d_for_field(&vy);

    let max_distance = torus_distance(&vx, &[0, 0], &[size / 2, size / 2]);

    let mut output_cos = vec![0.; output_size];
    let mut output_sin = vec![0.; output_size];
    let mut norm_for_output = vec![0.; output_size];
    let h = max_distance / (output_size as f64);

    let mut index;

    for i_first_point in 0..size {
        for j_first_point in 0..size {
            let point_1 = [i_first_point, j_first_point];
            let angle_1 = angle_for_vector_field(
                &vx[i_first_point][j_first_point],
                &vy[i_first_point][j_first_point],
            );

            for j_second_point in (j_first_point + 1)..size {
                let point_2 = [i_first_point, j_second_point];
                index = (torus_distance(&vx, &point_1, &point_2) / h).trunc() as usize;
                if index == output_size {
                    index -= 1;
                }
                let angle_2 = angle_for_vector_field(
                    &vx[i_first_point][j_second_point],
                    &vy[i_first_point][j_second_point],
                );
                output_cos[index] += angle_1.cos() * angle_2.cos();
                output_sin[index] += angle_1.sin() * angle_2.sin();
                norm_for_output[index] += 1.;
            }

            for i_second_point in (i_first_point + 1)..size {
                for j_second_point in 0..size {
                    let point_2 = [i_second_point, j_second_point];
                    index = (torus_distance(&vx, &point_1, &point_2) / h).trunc() as usize;
                    if index == output_size {
                        index -= 1;
                    }
                    let angle_2 = angle_for_vector_field(
                        &vy[i_second_point][j_second_point],
                        &vy[i_second_point][j_second_point],
                    );
                    output_cos[index] += angle_1.cos() * angle_2.cos();
                    output_sin[index] += angle_1.sin() * angle_2.sin();
                    norm_for_output[index] += 1.;
                }
            }
        }
    }
    for i in 0..output_size {
        output_cos[i] = output_cos[i] / norm_for_output[i];
        output_sin[i] = output_sin[i] / norm_for_output[i];
    }
    [output_cos, output_sin, norm_for_output]
}
