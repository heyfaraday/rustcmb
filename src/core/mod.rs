pub struct Field {
    pub size: usize,
    pub vec: Vec<Vec<f64>>,
}

pub struct Coef {
    pub size: usize,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
}

impl Field {
    pub fn new(size: usize) -> Field {
        Field {
            size: size,
            vec: vec![vec![0.; size + 1]; size + 1],
        }
    }
}

impl Coef {
    pub fn new(size: usize) -> Coef {
        Coef {
            size: size,
            a: vec![vec![0.; size]; size / 2 + 1],
            b: vec![vec![0.; size]; size / 2 + 1],
        }
    }
}
