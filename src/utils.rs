use std::f32;

pub fn vec_min(vec: &Vec<f32>) -> f32 {
    vec.iter().fold(f32::INFINITY, |a, &b| a.min(b))
}

pub fn vec_max(vec: &Vec<f32>) -> f32 {
    vec.iter().fold(f32::NAN, |a, &b| b.max(a))
}

pub fn vec_avg(vec: &Vec<f32>) -> f32 {
    let avg = vec.iter().fold(0.0, |acc, x| acc + x);
    avg / vec.len() as f32
}