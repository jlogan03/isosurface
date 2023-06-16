use crate::{
    distance::Signed,
    math::Vec3,
    source::{CentralDifference, HermiteSource, ScalarSource},
};
use std::f32::MAX;

/// A canonical single gyroid
/// with distance metric from thresholded minimal surface
struct GyroidScalar {
    /// The threshold value of the distance function,
    /// which determines thickness
    pub threshold: f32,
    /// The (xmax, ymax, zmax) corner of the bounding box to populate
    pub span: (f32, f32, f32),
}

impl GyroidScalar {
    /// Create a gyroid with the specified threshold and span
    pub fn new(threshold: f32, span: (f32, f32, f32)) -> Self {
        Self { threshold, span }
    }
}

impl ScalarSource for GyroidScalar {
    fn sample_scalar(&self, p: Vec3) -> Signed {
        let (x, y, z) = p.0;
        let v = x.sin() * y.cos() + z.sin() * x.cos() + y.sin() * z.cos() + self.threshold;
        Signed(v)
    }
}

/// A canonical single gyroid
/// with distance metric from thresholded minimal surface
/// and with normals from finite difference
pub struct Gyroid(CentralDifference<GyroidScalar>);

impl Gyroid {
    /// Create a gyroid with the specified threshold and span
    pub fn new(threshold: f32, span: (f32, f32, f32), epsilon: f32) -> Self {
        let gyroid_scalar = GyroidScalar { threshold, span };
        let finite_difference = CentralDifference::new_with_epsilon(gyroid_scalar, epsilon);
        Self(finite_difference)
    }
}

impl ScalarSource for Gyroid {
    fn sample_scalar(&self, p: Vec3) -> Signed {
        self.0.sample_scalar(p)
    }
}

impl HermiteSource for Gyroid {
    fn sample_normal(&self, p: Vec3) -> Vec3 {
        self.0.sample_normal(p)
    }
}
