
use vec3::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {
    pub fn new(a_: Vec3, b_: Vec3) -> Self { Ray{a: a_, b: b_} }

    pub fn origin(self) -> Vec3 {
        self.a
    }

    pub fn direction(self) -> Vec3 {
        self.b
    }

    pub fn point_of_parameter(self, t: f64) -> Vec3 {
        self.a + self.b.mul_by_float(t)
    }
}
