use ray::Ray;
use vec3::{Vec3, dot};
use hitable::{Hitable, HitRecord};

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = *r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(&oc, r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp  > t_min {
                rec.t = temp;
                rec.p = r.point_of_parameter(rec.t);
                rec.normal = (rec.p - self.center).div_by_float(self.radius);

                return true;
            }
            temp = (-b + (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_of_parameter(rec.t);
                rec.normal = (rec.p - self.center).div_by_float(self.radius);

                return true;
            }
        }

        false

    }
}