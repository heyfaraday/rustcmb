#[derive(Default, Debug)]
pub struct Field {
    size: usize,
    vec: Vec<Vec<f64>>,
}

impl Field {
    pub fn new(size: usize) -> Field {
        Field {
            size: size,
            vec: vec![vec![0.; size / 2 + 1]; size + 1],
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
        self.size / 2 + 1
    }
}


#[derive(Default, Debug)]
pub struct Coef {
    size: usize,
    a: Vec<Vec<f64>>,
    b: Vec<Vec<f64>>,
}

// всё перепроверить, написать error handling для найквиста и l and m
impl Coef {
    pub fn new(size: usize) -> Coef {
        Coef {
            size: size,
            a: vec![vec![0.; size]; size],
            b: vec![vec![0.; size]; size],
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn a_at(&self, l: usize, m: usize) -> &f64 {
        &self.a[l][m]
    }

    #[inline]
    pub fn a_at_mut(&mut self, l: usize, m: usize) -> &mut f64 {
        &mut self.a[l][m]
    }

    #[inline]
    pub fn b_at(&self, l: usize, m: usize) -> &f64 {
        &self.b[l][m]
    }

    #[inline]
    pub fn b_at_mut(&mut self, l: usize, m: usize) -> &mut f64 {
        &mut self.b[l][m]
    }

    #[inline]
    pub fn a_l_begin(&self) -> usize {
        0
    }

    #[inline]
    pub fn a_l_end(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn a_m_begin(&self) -> usize {
        0
    }

    #[inline]
    pub fn a_m_end(&self, l: usize) -> usize {
        l + 1
    }

    #[inline]
    pub fn b_l_begin(&self) -> usize {
        0
    }

    #[inline]
    pub fn b_l_end(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn b_m_begin(&self) -> usize {
        1
    }

    #[inline]
    pub fn b_m_end(&self, l: usize) -> usize {
        l + 1
    }
}
