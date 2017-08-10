extern crate rustfft;

use self::rustfft::algorithm::Radix4;
use self::rustfft::FFT;
use self::rustfft::num_complex::Complex;
use self::rustfft::num_traits::Zero;

pub fn d_1d(field: &mut Vec<f64>, a_mods: &Vec<f64>, b_mods: &Vec<f64>) {

    let size = field.capacity() - 1;

    assert!(field.capacity() == size + 1);
    assert!(a_mods.capacity() == size / 2 + 1);
    assert!(b_mods.capacity() == size / 2 + 1);
    assert!(b_mods[0] == 0.);
    assert!(b_mods[size / 2] == 0.);

    let mut input: Vec<Complex<f64>> = vec![Complex::zero(); size];
    let mut output: Vec<Complex<f64>> = vec![Complex::zero(); size];

    for i in 0..(size / 2 + 1) {
        input[i] = Complex::new(i as f64 * b_mods[i], -(i as f64) * a_mods[i]);
    }

    let fft = Radix4::new(size, false);
    fft.process(&mut input, &mut output);

    for i in 0..(size) {
        field[i] = output[i].re;
    }
    field[size] = output[0].re;
}
