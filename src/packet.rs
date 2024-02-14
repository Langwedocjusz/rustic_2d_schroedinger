//Utility to generate a wave packet solutions to
//Schroedinger's equation with given parameters

use crate::complex::*;

//Returns a double fourier transform of a 2d gaussian centered around
//'momentum':
pub fn gaussian_packet(x: f64, y: f64, width: f64, momentum: (f64, f64)) -> Complex64 {
    width
    *
    f64::exp(-0.5*width*x*x) * Complex64::expi(momentum.0*x)
    *
    f64::exp(-0.5*width*y*y) * Complex64::expi(momentum.1*y)
}