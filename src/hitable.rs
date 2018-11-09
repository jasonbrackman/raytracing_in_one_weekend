use ray::Ray;
use vec3::Vec3;
use material;


#[derive(Copy,Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: material::Lambertian
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::new(),
            normal: Vec3::new(),
            material: material::Lambertian{albedo: Vec3::new()}
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}