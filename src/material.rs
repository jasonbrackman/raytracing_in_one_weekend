use vec3::{Vec3, unit_vector, dot};
use ray::Ray;
use sphere;
use hitable::HitRecord;


pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

#[derive(Copy,Clone)]
#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + sphere::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target-rec.p);
        *attenuation = self.albedo;

        true
    }
}

#[derive(Copy,Clone)]
pub struct Metal {
    pub albedo:Vec3
}

impl Metal {
   fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * (2.0 * dot(v, n))
   }
}


impl Material for Metal {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Metal::reflect(&unit_vector(*r_in.direction()), &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        let result = dot(scattered.direction(), &rec.normal) > 0.0;

        result
    }
}