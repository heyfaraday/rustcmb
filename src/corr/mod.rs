use err::{size_assert_2d_for_field, point_assert};
use math::torus_distance;

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
                output[index] += field[i_first_point][j_first_point] *
                    field[i_first_point][j_second_point];
                norm_for_output[index] += 1.;
            }

            for i_second_point in (i_first_point + 1)..size {
                for j_second_point in 0..size {

                    let point_2 = [i_second_point, j_second_point];
                    index = (torus_distance(&field, &point_1, &point_2) / h).trunc() as usize;
                    if index == output_size {
                        index -= 1;
                    }
                    output[index] += field[i_first_point][j_first_point] *
                        field[i_second_point][j_second_point];
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
