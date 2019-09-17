use hitable::HitRecord;
use rand::random;
use ray::Ray;
use sphere;
use vec3::{dot, unit_vector, Vec3};

pub trait Material: MaterialClone {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

// this is coming from the examples provided here:
// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
// -- without this I had no way to of properly assigning the hitrecord's material to the ray object
// -- as it was always already in a borrowed state and couldn't be moved again.  So I needed a way
// -- to copy/clone the material.
pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}
impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(r: f32, g: f32, b: f32) -> Box<Lambertian> {
        Box::new(Lambertian {
            albedo: Vec3 { e: [r, g, b] },
        })
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p + rec.normal + sphere::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;

        true
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(r: f32, g: f32, b: f32, fuzz: f32) -> Box<Metal> {
        let fuzziness = if fuzz > 1.0 { 1.0 } else { fuzz };

        Box::new(Metal {
            albedo: Vec3 { e: [r, g, b] },
            fuzz: fuzziness,
        })
    }
    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * (2.0 * dot(v, n))
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Metal::reflect(&unit_vector(*r_in.direction()), &rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + sphere::random_in_unit_sphere() * self.fuzz,
        );
        *attenuation = self.albedo;

        dot(scattered.direction(), &rec.normal) > 0.0
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Box<Dielectric> {
        Box::new(Dielectric { refraction_index })
    }

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * (2.0 * dot(v, n))
    }

    fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
        let uv = unit_vector(*v);
        let dt = dot(&uv, n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            *refracted = (uv - *n * dt) * ni_over_nt - *n * discriminant.sqrt();
            true
        } else {
            false
        }
    }

    fn schlick(cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        // setup some temp vars
        let outward_normal;
        let reflected = Dielectric::reflect(r_in.direction(), &rec.normal);
        let ni_over_nt;

        *attenuation = Vec3 { e: [1.0, 1.0, 1.0] };

        let refracted = &mut Vec3::new();
        let reflect_probability;
        let cosine = if dot(r_in.direction(), &rec.normal) > 0.0 {
            outward_normal = rec.normal * -1.0;
            ni_over_nt = self.refraction_index;

            self.refraction_index * dot(r_in.direction(), &rec.normal) / (r_in.direction().length())
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.refraction_index;

            dot(r_in.direction(), &rec.normal) * -1.0 / (r_in.direction().length())
        };

        if Dielectric::refract(r_in.direction(), &outward_normal, ni_over_nt, refracted) {
            reflect_probability = Dielectric::schlick(cosine, self.refraction_index);
        //println!("Yes.");
        } else {
            reflect_probability = 1.0;
        }

        if random::<f32>() < reflect_probability {
            *scattered = Ray::new(rec.p, reflected.clone());
        } else {
            *scattered = Ray::new(rec.p, refracted.clone());
        }

        true
    }
}
