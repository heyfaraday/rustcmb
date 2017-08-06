extern crate text_io;

use std::fs::File;
use std::io::prelude::Write;

pub fn write_1d(field: &Vec<f64>, folder: &str, name: &str) {

    let size = field.capacity() - 1;

    assert!(field.capacity() == size + 1);

    let file_name = [folder, "f.dat"].concat();
    let mut file = File::create(file_name).expect("Unable to create file");

    for i in 0..(size + 1) {
        file.write_all((format!("{}\t{}\n", i, field[i])).as_bytes())
            .expect("Unable to write data");
    }
}

pub fn write_2d(field: &Vec<Vec<f64>>, folder: &str, name: &str) {

    let size_1 = field.capacity() - 1;
    let size_2 = field[0].capacity() - 1;

    assert!(field.capacity() == size_1 + 1);
    assert!(field[0].capacity() == size_2 + 1);

    let file_name = [folder, "f.dat"].concat();
    let mut file = File::create(file_name).expect("Unable to create file");

    for i in 0..(size_1 + 1) {
        for j in 0..(size_2 + 1) {
            file.write_all((format!("{}\t{}\t{}\n", i, j, field[i][j])).as_bytes())
                .expect("Unable to write data");
        }
    }
}
