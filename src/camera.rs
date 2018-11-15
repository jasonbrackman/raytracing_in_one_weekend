use vec3::{Vec3, unit_vector, cross, dot};
use ray::Ray;
use rand::random;

pub fn random_in_unit_disk() -> Vec3 {
    let mut p = Vec3{e:[1.0, 1.0, 1.0]};
    while dot(&p, &p) >= 1.0 {
        p = Vec3{e:[random::<f32>(), random::<f32>(), 0.0]} * 2.0 - Vec3{e:[1.0, 1.0, 0.0]};
    }
    p
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, v_fov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = v_fov*3.14/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(&v_up, &w));
        let v = cross(&w, &u);

        let lower_left_corner = origin - (u * half_width * focus_dist) - (v * half_height * focus_dist) - w * focus_dist;
        let horizontal = u * half_width * focus_dist * 2.0;
        let vertical = v * half_height * focus_dist * 2.0;

        Camera {origin, lower_left_corner, horizontal, vertical, u, v, lens_radius}
    }

    pub fn get_ray(&self, s: f32, t:f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() +  self.v * rd.y();
        Ray::new(self.origin + offset,
        self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset)
    }
}