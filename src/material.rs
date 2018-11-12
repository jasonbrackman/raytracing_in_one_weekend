use vec3::{Vec3, unit_vector, dot};
use ray::Ray;
use sphere;
use hitable::HitRecord;


pub trait Material: MaterialClone {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}


// this is coming from the examples provided here:
// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
// -- without this I had no way to of properly assigning the hitrecord's material to the ray object
// -- as it was always already in a borrowed state and couldn't be moved again.  So I needed a way
// -- to copy/clone the material.
pub trait MaterialClone {
    fn clone_box(&self) -> Box<Material>;
}
impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Material> {
    fn clone(&self) -> Box<Material> {
        self.clone_box()
    }
}

#[derive(Copy,Clone, Debug)]
pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + sphere::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target-rec.p);
        *attenuation = self.albedo;

        true
    }
}

#[derive(Copy,Clone, Debug)]
pub struct Metal {
    pub albedo:Vec3,
    pub fuzz:f64
}

impl Metal {
    pub fn new(r:f64, g:f64, b:f64, fuzz:f64) -> Metal {
        let fuzziness = match fuzz > 1.0 {
            true => 1.0,
            false => fuzz
        };

        Metal{albedo: Vec3{e: [r, g, b]}, fuzz: fuzziness}
    }
    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * (2.0 * dot(v, n))
    }
}


impl Material for Metal {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Metal::reflect(&unit_vector(*r_in.direction()), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + sphere::random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;

        dot(scattered.direction(), &rec.normal) > 0.0
    }
}