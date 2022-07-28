mod approx_eq;
mod convert;
mod matrix;
mod quaternion;
mod vector;

#[inline(always)]
pub fn sq(v: f64) -> f64 {
    v * v
}
