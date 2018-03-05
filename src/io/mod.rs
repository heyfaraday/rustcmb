extern crate text_io;

use std::fs::File;
use std::io::prelude::Write;
use std::io::{BufRead, BufReader};

pub fn write_1d(field: &Vec<f64>, folder: &str, name: &str) {
    let size = field.capacity() - 1;

    let file_name = [folder, name].concat();
    let mut file = File::create(file_name).expect("Unable to create file");

    for i in 0..(size + 1) {
        file.write_all((format!("{}\t{}\n", i, field[i])).as_bytes())
            .expect("Unable to write data");
    }
}

pub fn write_2d(field: &Vec<Vec<f64>>, folder: &str, name: &str) {
    let size_1 = field.capacity() - 1;
    let size_2 = field[0].capacity() - 1;

    let file_name = [folder, name].concat();
    let mut file = File::create(file_name).expect("Unable to create file");

    for i in 0..(size_1 + 1) {
        for j in 0..(size_2 + 1) {
            file.write_all((format!("{}\t{}\t{}\n", i, j, field[i][j])).as_bytes())
                .expect("Unable to write data");
        }
    }
}

pub fn read_1d(data: &mut Vec<f64>, folder: &str, name: &str) {
    let file_name = [folder, name].concat();
    let file = BufReader::new(File::open(file_name).expect("Unable to open file"));

    let mut arr = vec![0.; 2];
    for (_, line) in file.lines().enumerate() {
        for (j, number) in line.unwrap().split("\t").enumerate() {
            arr[j] = number.trim().parse().unwrap();
        }
        data[arr[0] as usize] = arr[1];
    }
}

pub fn read_2d(data: &mut Vec<Vec<f64>>, folder: &str, name: &str) {
    let file_name = [folder, name].concat();
    let file = BufReader::new(File::open(file_name).expect("Unable to open file"));

    let mut arr = vec![0.; 3];
    for (_, line) in file.lines().enumerate() {
        for (j, number) in line.unwrap().split("\t").enumerate() {
            arr[j] = number.trim().parse().unwrap();
        }
        data[arr[0] as usize][arr[1] as usize] = arr[2];
    }
}
