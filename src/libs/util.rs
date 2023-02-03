extern crate nalgebra_glm as glm;
use glm::{DVec3, TVec};

fn normalize(x: &DVec3) -> DVec3 {
    let nrm: DVec3 = norm(x);
    let cln:DVec3 = DVec3::new(x[0], x[1], x[2]);

    return DVec3::new(cln[0]/nrm[0], cln[1]/nrm[1], cln[2]/nrm[2]);
}

fn norm_sq(x: &DVec3) -> f64 {
    x.dot(x)
}

fn norm(x: &DVec3) -> DVec3 {
    return glm::normalize(x);
}

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

fn to_vec3_from_float(x: f64) -> DVec3 {
    glm::vec3(x, x, x)
}

fn to_str(x: bool) -> String {
    if x {
        return String::from("1");
    } else {
        return String::from("0");
    }
}

fn slice_to_str(x: &str) -> String {
    return String::from("_") + x;
}

fn vec3_str(x: &DVec3) -> String {
    String::from("vec3(")
        + &x[0].to_string()[..]
        + ","
        + &x[1].to_string()[..]
        + ","
        + &x[2].to_string()[..]
        + ")"
}
