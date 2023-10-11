use fastrand;

pub fn random() -> f64 {
    fastrand::f64()
}

pub fn random_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}
