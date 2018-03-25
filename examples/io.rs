extern crate rustcmb;

use rustcmb::io::*;

#[allow(dead_code)]
const DATA_IN: &str = "../data/in/rust-examples/io/";
const DATA_OUT: &str = "../data/out/rust-examples/io/";

fn main() {
    let mut field_2d = vec![vec![1.2; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            field_2d[i][j] = i as f64 * 10. + j as f64;
        }
    }
    write_2d(&field_2d, &DATA_OUT, &"2d_test.dat");
    read_2d(&mut field_2d, &DATA_OUT, &"2d_test.dat");
    println!("{:?}", field_2d);

    let mut field_1d = vec![1.2; 5];
    for i in 0..5 {
        field_1d[i] = i as f64;
    }
    write_1d(&field_1d, &DATA_OUT, &"1d_test.dat");
    read_1d(&mut field_1d, &DATA_OUT, &"1d_test.dat");
    println!("{:?}", field_1d);
}
