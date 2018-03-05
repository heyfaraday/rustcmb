extern crate rustcmb;

use rustcmb::core::sphere;

const SIZE: usize = 4;

fn main() {
    let default_field = sphere::Field::default();
    let field = sphere::Field::new(SIZE);
    let mut mut_field = sphere::Field::new(SIZE);

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

    let mut foo_coef = sphere::Coef::new(SIZE);

    for l in foo_coef.a_l_begin()..foo_coef.a_l_end() {
        for m in foo_coef.a_m_begin()..foo_coef.a_m_end(l) {
            *foo_coef.a_at_mut(l, m) = (l + m + 5) as f64;
        }
    }

    for i in foo_coef.b_l_begin()..foo_coef.b_l_end() {
        for j in foo_coef.b_m_begin()..foo_coef.b_m_end(i) {
            *foo_coef.b_at_mut(i, j) = (i + j + 10) as f64;
        }
    }

    println!("foo_coef after simple indexing: {:?}", foo_coef);
}
