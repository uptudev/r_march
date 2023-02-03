extern crate nalgebra_glm as glm;

use glm::{DVec3, TVec};

/// **normalize**
/// 
/// See xq_norm()
fn normalize(x: &DVec3) -> DVec3 {
    let nrm: DVec3 = norm(x);
    let cln:DVec3 = DVec3::new(x[0], x[1], x[2]);

    return DVec3::new(cln[0]/nrm[0], cln[1]/nrm[1], cln[2]/nrm[2]);
}

/// **xq_norm**
/// 
/// Runs a vector for `x / norm(x)`, which returns `x` multiplied by the inverse of normalized `x`.
fn xq_norm(x: &DVec3) -> DVec3 {
    normalize(x)
}

/// **norm_sq**
/// 
/// See dot_product()
fn norm_sq(x: &DVec3) -> f64 {
    x.dot(x)
}

/// **dot_product**
/// 
/// Gets the dot product for a given vector.
fn dot_product(x: &DVec3) -> f64 {
    norm_sq(x)
}

/// **norm**
/// 
/// Gets a normalized vector.
fn norm(x: &DVec3) -> DVec3 {
    return glm::normalize(x);
}

/// **get_sub_keys**
/// 
/// Gets sub keys pushed from GLSL.
fn get_sub_keys<T, const R: usize>(v: &TVec<T, R>) -> Vec<String>
where
    T: std::fmt::Display,
{
    let mut ret = Vec::new();
    let mut i: usize = 0;
    while i < R {
        ret.push(v[i].to_string());
        i += 1;
    }
    ret
}

/// **to_vec3_from_f64**
/// 
/// Returns a given 3D vector populated with a given `f64` in all 3 axes.
fn to_vec3_from_f64(x: f64) -> DVec3 {
    glm::vec3(x, x, x)
}

/// **to_str**
/// 
/// Returns a boolean represented as a String.
fn to_str(x: bool) -> String {
    if x {
        return String::from("1");
    } else {
        return String::from("0");
    }
}

/// **slice_to_string**
/// 
/// Returns a String populated by a `_` literal followed by the given string slice.
fn slice_to_str(x: &str) -> String {
    return String::from("_") + x;
}

/// **vec3_str**
/// 
/// Returns a string representation of a 3D vector.
fn vec3_str(x: &DVec3) -> String {
    String::from("vec3(")
        + &x[0].to_string()[..]
        + ","
        + &x[1].to_string()[..]
        + ","
        + &x[2].to_string()[..]
        + ")"
}
/// **q_half**
/// 
/// Returns 1/2 of a given float in a much faster way than doing `x/2`.
fn q_half(x:f64) -> f64{
    let i:u64 = x.to_bits(); // Deconstruct to bits
    f64::from_bits(i << 1)
}

/// **vec3_eq**
/// 
/// Returns true if vector `v` is populated with the same values as vector `w`.
fn vec3_eq(v:DVec3, w:DVec3) -> bool{
    let mut i = 0;
    while i < 3 {
        if v[i] != w[i] {
            return false;
        }
        i += 1;
    }
    return true;
}

/// **smin**
/// 
/// Does some math; will update when I find context other than identical shader fn
fn smin(a:f64, b:f64, k:f64) -> f64{
    f64::min(f64::max(0.5 + q_half(b-a)/k, 0.0), 1.0)
}