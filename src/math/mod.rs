mod approx_eq;
pub(crate) mod matrix;
pub(crate) mod quaternion;
pub(crate) mod vector;

#[inline(always)]
pub fn sq(v: f64) -> f64 {
    v * v
}
