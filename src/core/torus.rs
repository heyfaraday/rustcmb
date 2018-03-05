#[derive(Default, Debug)]
pub struct Field {
    size: usize,
    vec: Vec<Vec<f64>>,
}

impl Field {
    pub fn new(size: usize) -> Field {
        Field {
            size: size,
            vec: vec![vec![0.; size + 1]; size + 1],
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn at(&self, index_1: usize, index_2: usize) -> &f64 {
        &self.vec[index_1][index_2]
    }

    #[inline]
    pub fn at_mut(&mut self, index_1: usize, index_2: usize) -> &mut f64 {
        &mut self.vec[index_1][index_2]
    }

    #[inline]
    pub fn i_begin(&self) -> usize {
        0
    }

    #[inline]
    pub fn i_end(&self) -> usize {
        self.size + 1
    }

    #[inline]
    pub fn j_begin(&self) -> usize {
        0
    }

    #[inline]
    pub fn j_end(&self) -> usize {
        self.size + 1
    }
}

#[derive(Default, Debug)]
pub struct Coef {
    size: usize,
    a: Vec<Vec<f64>>,
    b: Vec<Vec<f64>>,
}

impl Coef {
    pub fn new(size: usize) -> Coef {
        Coef {
            size: size,
            a: vec![vec![0.; size]; size / 2 + 1],
            b: vec![vec![0.; size]; size / 2 + 1],
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn a_at(&self, i: usize, j: usize) -> &f64 {
        &self.a[i][j]
    }

    #[inline]
    pub fn a_at_mut(&mut self, i: usize, j: usize) -> &mut f64 {
        &mut self.a[i][j]
    }

    #[inline]
    pub fn b_at(&self, i: usize, j: usize) -> &f64 {
        &self.b[i][j]
    }

    #[inline]
    pub fn b_at_mut(&mut self, i: usize, j: usize) -> &mut f64 {
        &mut self.b[i][j]
    }

    #[inline]
    pub fn a_i_begin(&self) -> usize {
        0
    }

    #[inline]
    pub fn a_i_end(&self) -> usize {
        self.size / 2 + 1
    }

    #[inline]
    pub fn a_j_begin(&self) -> usize {
        0
    }

    #[inline]
    pub fn a_j_end(&self, i: usize) -> usize {
        if i == 0 || i == self.size / 2 {
            self.size / 2 + 1
        } else {
            self.size
        }
    }

    #[inline]
    pub fn b_i_begin(&self) -> usize {
        0
    }

    #[inline]
    pub fn b_i_end(&self) -> usize {
        self.size / 2 + 1
    }

    #[inline]
    pub fn b_j_begin(&self, i: usize) -> usize {
        if i == 0 || i == self.size / 2 {
            1
        } else {
            0
        }
    }

    #[inline]
    pub fn b_j_end(&self, i: usize) -> usize {
        if i == 0 || i == self.size / 2 {
            self.size / 2
        } else {
            self.size
        }
    }
}
