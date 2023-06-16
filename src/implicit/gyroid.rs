use crate::{
    distance::{Directed, Signed},
    math::Vec3,
    source::{HermiteSource, ScalarSource, VectorSource},
};

/// A canonical single gyroid
/// with distance metric from thresholded minimal surface
pub struct Gyroid {
    /// The threshold value of the distance function,
    /// which determines thickness
    pub threshold: f32,
    /// The scale applied to the full unit cell
    pub scale: f32
}

impl Gyroid {
    /// Create a gyroid with the specified threshold and span
    pub fn new(threshold: f32, scale: f32) -> Self {
        Self { threshold, scale }
    }
}

impl ScalarSource for Gyroid {
    fn sample_scalar(&self, p: Vec3) -> Signed {
        let p = p / self.scale;
        let (x, y, z) = (p.x, p.y, p.z);
        let v = x.sin() * y.cos() + z.sin() * x.cos() + y.sin() * z.cos() + self.threshold;
        Signed(v)
    }
}

impl VectorSource for Gyroid {
    fn sample_vector(&self, p: Vec3) -> Directed {
        Directed(Vec3::new(0.0, 0.0, 0.0))
    }
}

impl HermiteSource for Gyroid {
    fn sample_normal(&self, p: Vec3) -> Vec3 {
        let p = p / self.scale;
        let (x, y, z) = (p.x, p.y, p.z);
        let ddx = x.cos() * y.cos() - z.sin() * x.sin();
        let ddy = (-x.sin() * y.sin()) + y.cos() * z.cos();
        let ddz = z.cos() * x.cos() - y.sin() * z.sin();

        Vec3::new(ddx, ddy, ddz)
    }
}
