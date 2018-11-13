use vec3::{Vec3, unit_vector, cross};
use ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, v_fov: f64, aspect: f64) -> Camera {
        let theta = v_fov*3.14/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(&v_up, &w));
        let v = cross(&w, &u);

        let lower_left_corner = origin - (u * half_width) - (v * half_height) - w;
        let horizontal = u * half_width * 2.0;
        let vertical = v * half_height * 2.0;

        Camera {origin, lower_left_corner, horizontal, vertical}
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner +
            self.horizontal * u  +
            self.vertical * v -
            self.origin)
    }
}