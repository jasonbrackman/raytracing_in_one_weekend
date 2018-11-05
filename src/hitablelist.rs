use ray::Ray;
use hitable::{Hitable, HitRecord};
use sphere::Sphere;
use vec3::Vec3;

pub struct HitableList<'a> {
    pub hit_records: Vec<&'a Sphere>
}

//impl <'a> HitableList<'a> {
////    fn new(hit_records:Vec<&'a Sphere> ) -> HitableList<'a> {
////        HitableList { hit_records }
////    }
//}

impl <'a> Hitable for HitableList<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        let mut temp_rec = HitRecord{
            t:0.0,
            p: Vec3{e:[0.0,0.0,0.0]},
            normal:Vec3{e:[0.0, 0.0, 0.0]}
        };
        for &item in self.hit_records.iter() {

            if item.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
            }
        }

        hit_anything
    }
}