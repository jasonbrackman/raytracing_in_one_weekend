
use vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self { Ray{ origin, direction} }

    pub fn origin(&self) -> &Vec3 {

        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {

        &self.direction
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {

        self.origin + self.direction.mul_by_float(t)
    }
}
