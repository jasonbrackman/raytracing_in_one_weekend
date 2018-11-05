extern crate rand;
use rand::{random};

use std::io::{Write, Result};
use std::fs::File;

mod hitable;
use hitable::{Hitable, HitRecord};

mod hitablelist;
use hitablelist::HitableList;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod sphere;
use sphere::Sphere;

mod camera;

fn hit_sphere(center: Vec3, radius:f64, r:&Ray) -> f64 {

    let origin_at_center = *r.origin() - center;
    let a = vec3::dot(r.direction(), r.direction());
    let b = 2.0 * vec3::dot(&origin_at_center, r.direction());
    let c = vec3::dot(&origin_at_center, &origin_at_center) - radius * radius;
    let discriminant = (b * b) - (4.0 * a * c);
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / 2.0 * a
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3{e:[1.0, 1.0, 1.0]};
    while p.squared_length() > 1.0 {
        p = Vec3{e:[random::<f64>(), random::<f64>(), random::<f64>()]}.mul_by_float(2.0) - Vec3{e:[1.0, 1.0, 1.0]};
    };

    p
}

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    let maxfloat = 10.0 * 100000000000.0;
    let rec = &mut HitRecord::new();
    if world.hit(r, 0.0, maxfloat, rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();

        let temp_r = Ray::new(rec.p, target-rec.p);
        return color( &temp_r, world).mul_by_float(0.5) ;

    } else {
        let unit_direction = vec3::unit_vector(*r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3 { e: [1.0, 1.0, 1.0] }.mul_by_float(1.0 - t) + Vec3 { e: [0.5, 0.7, 1.0] }.mul_by_float(t)
    }
}

fn render_ppm() -> Result<()> {

    let mut buffer = File::create("helloworld.ppm")?;


    let nx = 200;
    let ny = 100;
    let ns = 100;

    write!(buffer, "P3\n{} {}\n255\n", nx, ny);

    let sphere_01 = Sphere{center:Vec3{e:[0.0, 0.0, -1.0]}, radius:0.5};
    let sphere_02 = Sphere{center:Vec3{e:[0.0, -100.5, -1.0]}, radius:100.0};

    let objects = vec!(
        &sphere_01,
        &sphere_02);

    let world = hitablelist::HitableList{hit_records: objects};
    let cam = &camera::Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3{e:[0.0, 0.0, 0.0]};
            for _s in 0..ns {
                let u = (i as f64 + random::<f64>()) / nx as f64;
                let v = (j as f64 + random::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                col = col + color(&r, &world);
            }

            col = col.div_by_float(ns as f64);
            col = Vec3{e:[col.r().sqrt(), col.g().sqrt(), col.b().sqrt()]};

            let ir = (255.99 * col.e[0]) as i64;
            let ig = (255.99 * col.e[1]) as i64;
            let ib = (255.99 * col.e[2]) as i64;

            write!(buffer, "{} {} {}\n", ir, ig, ib);
        }
    }
    Ok(())
}

fn main() {
    //random_in_unit_sphere();
    let _x = render_ppm().unwrap();
}
