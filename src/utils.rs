pub const INF: f64 = 0xfffffff as f64;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    rand::random::<f64>()
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

pub fn random_i32_range(min: i32, max: i32) -> i32 {
    random_f64_range(min as f64, (max + 1) as f64) as i32
}

pub fn fmax(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn fmin(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    let mut res = x;
    if x < min {
        res = min;
    }
    if x > max {
        res = max;
    }

    res
}
