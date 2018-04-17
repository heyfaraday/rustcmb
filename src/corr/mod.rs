use err::{point_assert, size_assert_2d_for_field};
use math::{angle_for_vector_field, torus_distance, torus_distance_square};

pub fn correlation_function(field: &Vec<Vec<f64>>, output_size: usize) -> [Vec<f64>; 2] {
    let size = field.len() - 1;

    size_assert_2d_for_field(&field);

    let max_distance = torus_distance(&field, &[0, 0], &[size / 2, size / 2]);

    let mut output = vec![0.; output_size];
    let mut norm_for_output = vec![0.; output_size];
    let h = max_distance / (output_size as f64);

    for i_first_point in 0..size {
        for j_first_point in 0..size {
            let point_1 = [i_first_point, j_first_point];

            for j_second_point in (j_first_point + 1)..size {
                let point_2 = [i_first_point, j_second_point];
                let mut index = (torus_distance(&field, &point_1, &point_2) / h).trunc() as usize;
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
                    let mut index =
                        (torus_distance(&field, &point_1, &point_2) / h).trunc() as usize;
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

pub fn correlation_function_for_null(field: &Vec<Vec<f64>>) -> [f64; 2] {
    let size = field.len() - 1;

    size_assert_2d_for_field(&field);

    let mut output: f64 = 0.;
    let mut norm_for_output: f64 = 0.;

    for i in 0..size {
        for j in 0..size {
            output += field[i][j] * field[i][j];
            norm_for_output += 1.;
        }
    }
    [output / norm_for_output, norm_for_output]
}

pub fn correlation_check(field: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let size = field.len() - 1;

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
    let size = field.len() - 1;

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
    let size = vx.len() - 1;
    let size_2 = vy.len() - 1;
    assert!(size == size_2);

    size_assert_2d_for_field(&vx);
    size_assert_2d_for_field(&vy);

    let max_distance = torus_distance(&vx, &[0, 0], &[size / 2, size / 2]);

    let mut output_cos = vec![0.; output_size];
    let mut output_sin = vec![0.; output_size];
    let mut norm_for_output = vec![0.; output_size];
    let h = max_distance / (output_size as f64);

    for i_first_point in 0..size {
        for j_first_point in 0..size {
            let point_1 = [i_first_point, j_first_point];
            let angle_1 = angle_for_vector_field(
                &vx[i_first_point][j_first_point],
                &vy[i_first_point][j_first_point],
            );

            for j_second_point in (j_first_point + 1)..size {
                let point_2 = [i_first_point, j_second_point];
                let mut index = (torus_distance(&vx, &point_1, &point_2) / h).trunc() as usize;
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
                    let mut index = (torus_distance(&vx, &point_1, &point_2) / h).trunc() as usize;
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

pub fn correlation_function_vector_field_for_null(
    vx: &Vec<Vec<f64>>,
    vy: &Vec<Vec<f64>>,
) -> [f64; 3] {
    let size = vx.len() - 1;
    let size_2 = vy.len() - 1;
    assert!(size == size_2);

    size_assert_2d_for_field(&vx);
    size_assert_2d_for_field(&vy);

    let mut output_cos: f64 = 0.;
    let mut output_sin: f64 = 0.;
    let mut norm_for_output: f64 = 0.;

    for i in 0..size {
        for j in 0..size {
            let angle = angle_for_vector_field(&vx[i][j], &vy[i][j]);

            output_cos += angle.cos() * angle.cos();
            output_sin += angle.sin() * angle.sin();
            norm_for_output += 1.;
        }
    }
    [
        output_cos / norm_for_output,
        output_sin / norm_for_output,
        norm_for_output,
    ]
}

pub fn two_point_correlation_function(
    field1: &Vec<Vec<f64>>,
    field2: &Vec<Vec<f64>>,
    output_size: usize,
) -> Vec<f64> {
    let size1 = field1.len() - 1;
    let size2 = field2.len() - 1;
    assert!(size1 == size2);

    size_assert_2d_for_field(&field1);
    size_assert_2d_for_field(&field2);

    let max_distance = torus_distance(&field1, &[0, 0], &[size1 / 2, size1 / 2]);

    let mut output = vec![0.; output_size];
    let mut norm_for_output = vec![0.; output_size];
    let h = max_distance / (output_size as f64);

    for i_first_point in 0..size1 {
        for j_first_point in 0..size1 {
            let point_1 = [i_first_point, j_first_point];

            for j_second_point in (j_first_point + 1)..size1 {
                let point_2 = [i_first_point, j_second_point];
                let mut index = (torus_distance(&field1, &point_1, &point_2) / h).trunc() as usize;
                if index == output_size {
                    index -= 1;
                }
                output[index] +=
                    field1[i_first_point][j_first_point] * field2[i_first_point][j_second_point];
                norm_for_output[index] += 1.;
            }

            for i_second_point in (i_first_point + 1)..size1 {
                for j_second_point in 0..size1 {
                    let point_2 = [i_second_point, j_second_point];
                    let mut index =
                        (torus_distance(&field1, &point_1, &point_2) / h).trunc() as usize;
                    if index == output_size {
                        index -= 1;
                    }
                    output[index] += field1[i_first_point][j_first_point]
                        * field2[i_second_point][j_second_point];
                    norm_for_output[index] += 1.;
                }
            }
        }
    }

    for i in 0..output_size {
        output[i] = output[i] / norm_for_output[i];
    }
    output
}

pub fn two_point_correlation_function_at_null(
    field1: &Vec<Vec<f64>>,
    field2: &Vec<Vec<f64>>,
) -> f64 {
    let size1 = field1.len() - 1;
    let size2 = field2.len() - 1;
    assert!(size1 == size2);

    size_assert_2d_for_field(&field1);
    size_assert_2d_for_field(&field2);

    let mut output: f64 = 0.;
    let mut norm_for_output: f64 = 0.;

    for i in 0..size1 {
        for j in 0..size1 {
            output += field1[i][j] * field2[i][j];
            norm_for_output += 1.;
        }
    }
    output / norm_for_output
}

pub fn two_point_correlation_function_with_all_distances(
    field1: &Vec<Vec<f64>>,
    field2: &Vec<Vec<f64>>,
) -> [Vec<f64>; 4] {
    let size1 = field1.len() - 1;
    let size2 = field2.len() - 1;
    assert!(size1 == size2);

    size_assert_2d_for_field(&field1);
    size_assert_2d_for_field(&field2);

    let initial_output_size = (size1 * size1 / 2) as usize;
    println!("initial_output_size: {:?}", initial_output_size);

    let max_distance = torus_distance_square(&field1, &[0, 0], &[size1 / 2, size1 / 2]);
    println!("max_distance: {:?}", max_distance);

    let mut initial_output = vec![0.; initial_output_size + 1];
    let mut initial_norm_for_output = vec![0.; initial_output_size + 1];

    for i_first_point in 0..size1 {
        for j_first_point in 0..size1 {
            let point_1 = [i_first_point, j_first_point];

            for j_second_point in (j_first_point + 1)..size1 {
                let point_2 = [i_first_point, j_second_point];
                let mut index = torus_distance_square(&field1, &point_1, &point_2) as usize;
                initial_output[index] +=
                    field1[i_first_point][j_first_point] * field2[i_first_point][j_second_point];
                initial_norm_for_output[index] += 1.;
            }

            for i_second_point in (i_first_point + 1)..size1 {
                for j_second_point in 0..size1 {
                    let point_2 = [i_second_point, j_second_point];
                    let mut index = torus_distance_square(&field1, &point_1, &point_2) as usize;
                    initial_output[index] += field1[i_first_point][j_first_point]
                        * field2[i_second_point][j_second_point];
                    initial_norm_for_output[index] += 1.;
                }
            }
        }
    }

    let mut output: Vec<f64> = Vec::new();
    let mut norm_for_output: Vec<f64> = Vec::new();
    let mut distances: Vec<f64> = Vec::new();

    for i in 0..(initial_output_size + 1) {
        if initial_norm_for_output[i] != 0. {
            output.push(initial_output[i] / initial_norm_for_output[i]);
            norm_for_output.push(initial_norm_for_output[i]);
            distances.push(i as f64);
        }
    }

    return [output, norm_for_output, distances, initial_norm_for_output];
}

pub fn two_point_correlation_function_2d(
    field1: &Vec<Vec<f64>>,
    field2: &Vec<Vec<f64>>,
    f: fn(f64, f64) -> f64,
) -> [Vec<Vec<f64>>; 2] {
    let size1 = field1.len() - 1;
    let size2 = field2.len() - 1;
    assert!(size1 == size2);

    size_assert_2d_for_field(&field1);
    size_assert_2d_for_field(&field2);

    let mut correlation: Vec<Vec<f64>> = vec![vec![0.; size1]; size1];
    let mut norm_for_correlation: Vec<Vec<f64>> = vec![vec![0.; size1]; size1];

    for i_first_point in 0..size1 {
        for j_first_point in 0..size1 {
            for j_second_point in (j_first_point + 1)..size1 {
                let i_index =
                    ((size1 / 2) as i32 + index(size1, i_first_point, i_first_point)) as usize;
                let j_index =
                    ((size1 / 2) as i32 + index(size1, j_first_point, j_second_point)) as usize;

                let i_index_2 =
                    ((size1 / 2) as i32 + index(size1, i_first_point, i_first_point)) as usize;
                let j_index_2 =
                    ((size1 / 2) as i32 + index(size1, j_second_point, j_first_point)) as usize;

                let corr = f(
                    field1[i_first_point][j_first_point],
                    field2[i_first_point][j_second_point],
                );

                correlation[i_index][j_index] += corr;
                correlation[i_index_2][j_index_2] += corr;
                norm_for_correlation[i_index][j_index] += 1.;
                norm_for_correlation[i_index_2][j_index_2] += 1.;
            }

            for i_second_point in (i_first_point + 1)..size1 {
                for j_second_point in 0..size1 {
                    let i_index =
                        ((size1 / 2) as i32 + index(size1, i_first_point, i_second_point)) as usize;
                    let j_index =
                        ((size1 / 2) as i32 + index(size1, j_first_point, j_second_point)) as usize;

                    let i_index_2 =
                        ((size1 / 2) as i32 + index(size1, i_second_point, i_first_point)) as usize;
                    let j_index_2 =
                        ((size1 / 2) as i32 + index(size1, j_second_point, j_first_point)) as usize;

                    let corr = f(
                        field1[i_first_point][j_first_point],
                        field2[i_second_point][j_second_point],
                    );

                    correlation[i_index][j_index] += corr;
                    correlation[i_index_2][j_index_2] += corr;
                    norm_for_correlation[i_index][j_index] += 1.;
                    norm_for_correlation[i_index_2][j_index_2] += 1.;
                }
            }
        }
    }

    for i_first_point in 0..size1 {
        for j_first_point in 0..size1 {
            let corr = f(
                field1[i_first_point][j_first_point],
                field2[i_first_point][j_first_point],
            );
            correlation[size1 / 2][size1 / 2] += corr;
            norm_for_correlation[size1 / 2][size1 / 2] += 1.;
        }
    }

    for i in 0..size1 {
        for j in 0..size1 {
            correlation[i][j] /= norm_for_correlation[i][j];
        }
    }

    [correlation, norm_for_correlation]
}

pub fn two_point_correlation_function_2d_dummy(
    field1: &Vec<Vec<f64>>,
    field2: &Vec<Vec<f64>>,
    f: fn(f64, f64) -> f64,
) -> [Vec<Vec<f64>>; 2] {
    let size1 = field1.len() - 1;
    let size2 = field2.len() - 1;
    assert!(size1 == size2);

    size_assert_2d_for_field(&field1);
    size_assert_2d_for_field(&field2);

    let mut correlation: Vec<Vec<f64>> = vec![vec![0.; size1]; size1];
    let mut norm_for_correlation: Vec<Vec<f64>> = vec![vec![0.; size1]; size1];

    for i_first_point in 0..size1 {
        for j_first_point in 0..size1 {
            for i_second_point in 0..size1 {
                for j_second_point in 0..size1 {
                    let i_index = index(size1, i_first_point, i_second_point);
                    let j_index = index(size1, j_first_point, j_second_point);

                    let i_index = ((size1 / 2) as i32 + i_index) as usize;
                    let j_index = ((size1 / 2) as i32 + j_index) as usize;

                    correlation[i_index][j_index] += f(
                        field1[i_first_point][j_first_point],
                        field2[i_second_point][j_second_point],
                    );
                    norm_for_correlation[i_index][j_index] += 1.;
                }
            }
        }
    }
    for i in 0..size1 {
        for j in 0..size1 {
            correlation[i][j] /= norm_for_correlation[i][j];
        }
    }

    [correlation, norm_for_correlation]
}

#[inline]
pub fn index(size: usize, first_index: usize, second_index: usize) -> i32 {
    if (second_index as i32 - first_index as i32) >= (size / 2) as i32 {
        (second_index as i32 - first_index as i32) - size as i32
    } else if (first_index as i32 - second_index as i32) > (size / 2) as i32 {
        (second_index as i32 - first_index as i32) + size as i32
    } else {
        second_index as i32 - first_index as i32
    }
}

#[inline]
pub fn index_i32(size: i32, first_index: i32, second_index: i32) -> i32 {
    if (second_index - first_index) >= (size / 2) {
        (second_index - first_index) - size
    } else if (first_index - second_index) > (size / 2) {
        (second_index - first_index) + size
    } else {
        second_index - first_index
    }
}

pub fn correlation_2d_to_1d_with_all_distances(correlation: &Vec<Vec<f64>>) -> [Vec<f64>; 4] {
    let size = correlation.len() - 1;

    size_assert_2d_for_field(&correlation);

    let initial_output_size = (size * size / 2) as usize;
    println!("initial_output_size: {:?}", initial_output_size);

    let mut initial_output = vec![0.; initial_output_size + 1];
    let mut initial_norm_for_output = vec![0.; initial_output_size + 1];

    for i in 0..size {
        for j in 0..size {
            let mut index =
                torus_distance_square(&correlation, &[size / 2, size / 2], &[i, j]) as usize;

            initial_output[index] += correlation[i][j];
            initial_norm_for_output[index] += 1.;
        }
    }

    let mut output: Vec<f64> = Vec::new();
    let mut norm_for_output: Vec<f64> = Vec::new();
    let mut distances: Vec<f64> = Vec::new();

    for i in 0..(initial_output_size + 1) {
        if initial_norm_for_output[i] != 0. {
            output.push(initial_output[i] / initial_norm_for_output[i]);
            norm_for_output.push(initial_norm_for_output[i]);
            distances.push(i as f64);
        }
    }

    [output, norm_for_output, distances, initial_norm_for_output]
}
