use hitable::{HitRecord, Hitable};
use material;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

pub struct HitableList<'a> {
    pub hit_records: &'a Vec<Sphere>,
}

//impl <'a> HitableList<'a> {
////    fn new(hit_records:Vec<&'a Sphere> ) -> HitableList<'a> {
////        HitableList { hit_records }
////    }
//}

impl<'a> Hitable for HitableList<'a> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        let mut temp_rec = HitRecord {
            t: 0.0,
            p: Vec3::new(),
            normal: Vec3::new(),
            material: material::Lambertian::new(1.0, 1.0, 1.0),
        };

        let mut closest_item = 0;
        for (index, item) in self.hit_records.iter().enumerate() {
            if item.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                closest_item = index;
            }
        }

        rec.t = temp_rec.t;
        rec.p = temp_rec.p;
        rec.normal = temp_rec.normal;
        rec.material = self.hit_records[closest_item].material.clone();

        hit_anything
    }
}
