use vec3::Vec3;
use ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: Vec3{e:[0.0, 0.0, 0.0]},
            lower_left_corner: Vec3{e:[-2.0, -1.0, -1.0]},
            horizontal: Vec3{e:[4.0, 0.0, 0.0]},
            vertical: Vec3{e:[0.0, 2.0, 0.0]}
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner +
            self.horizontal.mul_by_float(u) +
            self.vertical.mul_by_float(v) -
            self.origin)
    }
}