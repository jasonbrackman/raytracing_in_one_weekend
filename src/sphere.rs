use rand::random;

use ray::Ray;
use vec3::{Vec3, dot};
use hitable::{Hitable, HitRecord};
use material::{Material};

//#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = *r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(&oc, r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp  > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;

                return true;
            }
            temp = (-b + (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;

                return true;
            }
        }

        false

    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3{e:[5.0, 4.0, 3.0]};
    while p.squared_length() > 1.0 {
        p = Vec3{e:[random::<f32>(), random::<f32>(), random::<f32>()]} * 2.0 - Vec3{e:[1.0, 1.0, 1.0]};
    };

    p
}
