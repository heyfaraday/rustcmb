extern crate rustcmb;

use rustcmb::core::torus;

const SIZE: usize = 4;

fn main() {

    let default_field = torus::Field::default();
    let field = torus::Field::new(SIZE);
    let mut mut_field = torus::Field::new(SIZE);

    println!("default_field: {:?}", default_field);
    println!("field: {:?}", field);
    println!("mut_field: {:?}", mut_field);

    println!("field size: {:?}", field.size());

    println!("field at (2, 2): {:?}", field.at(2, 2));

    *mut_field.at_mut(2, 2) = 1.;
    println!("mut_field: {:?}", mut_field);

    for i in mut_field.i_begin()..mut_field.i_end() {
        for j in mut_field.j_begin()..mut_field.j_end() {
            *mut_field.at_mut(i, j) = (i + j) as f64;
        }
    }

    println!("mut_field after simple indexing: {:?}", mut_field);

    let mut foo_coef = torus::Coef::new(SIZE);

    for i in foo_coef.a_i_begin()..foo_coef.a_i_end() {
        for j in foo_coef.a_j_begin()..foo_coef.a_j_end(i) {
            *foo_coef.a_at_mut(i, j) = (i + j + 5) as f64;
        }
    }

    for i in foo_coef.b_i_begin()..foo_coef.b_i_end() {
        for j in foo_coef.b_j_begin(i)..foo_coef.b_j_end(i) {
            *foo_coef.b_at_mut(i, j) = (i + j + 10) as f64;
        }
    }

    println!("foo_coef after simple indexing: {:?}", foo_coef);
}
