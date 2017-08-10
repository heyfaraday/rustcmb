pub fn size_assert_2d_for_field(field: &Vec<Vec<f64>>) {

    let size = field.capacity() - 1;

    assert!(field.capacity() == size + 1);
    assert!(field[0].capacity() == size + 1);
}

pub fn size_assert_2d(field: &Vec<Vec<f64>>, a_mods: &Vec<Vec<f64>>, b_mods: &Vec<Vec<f64>>) {

    let size = field.capacity() - 1;

    assert!(field.capacity() == size + 1);
    assert!(field[0].capacity() == size + 1);
    assert!(a_mods.capacity() == size / 2 + 1);
    assert!(b_mods.capacity() == size / 2 + 1);
    assert!(a_mods[0].capacity() == size);
    assert!(b_mods[0].capacity() == size);
}

pub fn nyquist_assert_2d(field: &Vec<Vec<f64>>, a_mods: &Vec<Vec<f64>>, b_mods: &Vec<Vec<f64>>) {

    let size = field.capacity() - 1;

    size_assert_2d(&field, &a_mods, &b_mods);

    for j_mode in (size / 2 + 1)..size {
        assert!(a_mods[0][j_mode] == 0., "Nyquist error in a_mods[0] (2d)");
        assert!(
            a_mods[size / 2][j_mode] == 0.,
            "Nyquist error in a_mods[size / 2] (2d)"
        );
    }

    for j_mode in (size / 2)..size {
        assert!(b_mods[0][j_mode] == 0., "Nyquist error in b_mods[0] (2d)");
        assert!(
            b_mods[size / 2][j_mode] == 0.,
            "Nyquist error in b_mods[size / 2] (2d)"
        );
    }
    assert!(b_mods[0][0] == 0., "Nyquist error in b_mods[0] (2d)");
    assert!(
        b_mods[size / 2][0] == 0.,
        "Nyquist error in b_mods[size / 2] (2d)"
    );
}

pub fn point_assert(field: &Vec<Vec<f64>>, point: &[usize; 2]) {

    let size = field.capacity() - 1;

    assert!(point[0] < size + 1);
    assert!(point[1] < size + 1);
}
