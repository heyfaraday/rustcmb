pub fn d_1d(field: &Vec<Vec<f64>>, answer: &Vec<Vec<f64>>) {

    let size: usize = field.capacity() - 1;
    assert!(field.capacity() == size + 1);
    assert!(field[0].capacity() == size + 1);
    assert!(answer.capacity() == size + 1);
    assert!(answer[0].capacity() == size + 1);

}

pub fn d_2d_x(field: &Vec<Vec<f64>>, answer: &Vec<Vec<f64>>) {

    let size: usize = field.capacity() - 1;
    assert!(field.capacity() == size + 1);
    assert!(field[0].capacity() == size + 1);
    assert!(answer.capacity() == size + 1);
    assert!(answer[0].capacity() == size + 1);
}


pub fn d_2d_y(field: &Vec<Vec<f64>>, answer: &Vec<Vec<f64>>) {

    let size: usize = field.capacity() - 1;
    assert!(field.capacity() == size + 1);
    assert!(field[0].capacity() == size + 1);
    assert!(answer.capacity() == size + 1);
    assert!(answer[0].capacity() == size + 1);
}
