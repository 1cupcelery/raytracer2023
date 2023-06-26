use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn random_f64() -> f64 {
    let rnd = rand::thread_rng().gen_range(0.0..f64::MAX);
    rnd / (f64::MAX + 1.0)
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
