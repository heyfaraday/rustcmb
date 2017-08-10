use err::{size_assert_2d_for_field, point_assert};
use math::torus_distance;

pub fn correlation_function(field: &Vec<Vec<f64>>) {

    //    let size = field.capacity() - 1;

    size_assert_2d_for_field(&field);
    //
    //    for i_first_point in 0..size {
    //        for j_first_point in 0..size {
    //            for j_second_point in (j_first_point + 1)..size {}
    //            for i_second_point in (i_first_point + 1)..size {
    //                for j_second_point in 0..size {}
    //            }
    //        }
    //    }
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
