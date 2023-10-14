use fastrand;

pub fn random() -> f32 {
    fastrand::f32()
}

pub fn random_in_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random()
}
