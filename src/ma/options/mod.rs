/// Enum which contains methods for calculating the integral. Used in methods module
pub enum CalculatingMethod { NEWTON, TRAPEZOID, SIMPSON }


/// Structures for users for convenient use
#[derive(Copy, Clone)]
pub struct Limits {
    pub limit_a: f64,
    pub limit_b: f64,
}

impl Limits {
    pub fn new(&self, limit_a: &f64, limit_b: &f64) -> Limits {
        Limits {
            limit_a: *limit_a,
            limit_b: *limit_b,
        }
    }
}

pub struct Points {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}
