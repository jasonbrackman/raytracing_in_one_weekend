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
mod material;
use material::{Lambertian, Metal, Dielectric};


//fn hit_sphere(center: Vec3, radius:f64, r:&Ray) -> f64 {
//
//    let origin_at_center = *r.origin() - center;
//    let a = vec3::dot(r.direction(), r.direction());
//    let b = 2.0 * vec3::dot(&origin_at_center, r.direction());
//    let c = vec3::dot(&origin_at_center, &origin_at_center) - radius * radius;
//    let discriminant = (b * b) - (4.0 * a * c);
//    if discriminant < 0.0 {
//        -1.0
//    } else {
//        (-b - discriminant.sqrt()) / 2.0 * a
//    }
//}

fn color(r: &Ray, world: &HitableList, depth: i32) -> Vec3 {
    let maxfloat = 10.0 * 100000000000.0;
    let rec = &mut HitRecord::new();

    if world.hit(r, 0.002, maxfloat, rec) {
        // setup up temp vars
        let mut scattered = Ray::new(Vec3::new(), Vec3::new());
        let mut attenuation = Vec3::new();

        if depth < 50 && rec.material.scatter(r, rec, &mut attenuation, &mut scattered) {
            attenuation * color(&scattered, world, depth + 1)
        } else {
            Vec3::new()
        }
    } else {
        let unit_direction = vec3::unit_vector(*r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3 { e: [1.0, 1.0, 1.0] } * (1.0 - t) + Vec3 { e: [0.5, 0.7, 1.0] } * t
    }
}

fn get_spheres()  -> Vec<Sphere> {
   let sphere_01 = Sphere{
        center:Vec3{e:[0.0, 0.0, -1.0]},
        radius:0.5,
        material: Box::new(Lambertian::new(0.1, 0.2, 0.8))
    };

    let sphere_02 = Sphere{
        center:Vec3{e:[0.0, -100.5, -1.0]},
        radius:100.0,
        material: Box::new(Lambertian::new(0.8, 0.8, 0.0))
    };

    let sphere_03 = Sphere{
        center:Vec3{e:[1.0, 0.0, -1.0]},
        radius:0.5,
        material: Box::new(Metal::new(0.8, 0.6, 0.2, 0.3))
    };

    let sphere_04 = Sphere{
        center:Vec3{e:[-1.0, 0.0, -1.0]},
        radius:0.5,
        material: Box::new(Dielectric::new(1.5))  // air = 1,
                                                                   // glass = 1.3-1.7,
                                                                   // diamond = 2.4
    };

    let sphere_05 = Sphere{
        center:Vec3{e:[-1.0, 0.0, -1.0]},
        radius:-0.45,
        material: Box::new(Dielectric::new(1.5))
    };

    vec!(sphere_01, sphere_02, sphere_03, sphere_04, sphere_05)
}

fn random_scene() -> Vec<Sphere> {
    let n = 500;

    let mut scene = vec!();

    scene.push(Sphere{
        center:Vec3{e:[0.0, -1000.0, 0.0]},
        radius:1000.0,
        material: Box::new(Lambertian::new(0.5, 0.5, 0.5))
    });


    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Vec3{e:[a as f64 + 0.9 + random::<f64>(), 0.2, b as f64 + 0.9 + random::<f64>()]};
            if (center - Vec3 { e: [4.0, 0.2, 0.0] }).length() > 0.9 {
                match choose_mat {
                    // diffuse
                    0.0...0.8 => scene.push(
                        Sphere {
                            center,
                            radius: 0.2,
                            material: Box::new(Lambertian::new(random::<f64>() * random::<f64>(), random::<f64>() * random::<f64>(), random::<f64>() * random::<f64>()))
                        }),

                    // metal
                    0.80001...0.95 => scene.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal::new(0.5 * (1.0 + random::<f64>()), 0.5 * (1.0 + random::<f64>()), 0.5 * (1.0 + random::<f64>()), 0.5 * random::<f64>()))
                    }),

                    // glass
                    _ => scene.push(Sphere { center, radius: 0.2, material: Box::new(Dielectric::new(1.5))})
                };
            } else {};
        }
    }

    scene.push(Sphere{
        center:Vec3{e:[0.0, 1.0, 0.0]},
        radius:1.0,
        material: Box::new(Dielectric::new(1.5))
    });
    scene.push(Sphere{
        center:Vec3{e:[-4.0, 1.0, 0.0]},
        radius:1.0,
        material: Box::new(Lambertian::new(0.4, 0.2, 0.1))
    });
    scene.push(Sphere{
        center:Vec3{e:[4.0, 1.0, 0.0]},
        radius:1.0,
        material: Box::new(Metal::new(0.7, 0.6, 0.5, 0.0))
    });

    scene
}

fn render_ppm() -> Result<()> {

    let mut buffer = File::create("helloworld.ppm")?;


    let nx = 600;
    let ny = 300;
    let ns = 250;

    write!(buffer, "P3\n{} {}\n255\n", nx, ny);

    let objects = random_scene(); //get_spheres();

    let world = hitablelist::HitableList{hit_records: &objects};

    let look_from = Vec3{e:[-8.0, 2.0, 2.0]}; // Vec3{e:[-4.0, 2.0, 4.0]},
    let look_at = Vec3{e:[0.0, 0.0, -1.0]};
    let distance_to_focus = (look_from - look_at).length();
    let aperture = 0.22;

    let cam = &camera::Camera::new(
                                   look_from,
                                   look_at,
                                   Vec3{e:[0.0, 1.0, 0.0]},
                                   30.0,
                                   nx as f64 / ny as f64,
                                    aperture,
                                    distance_to_focus
    );

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new();
            for _s in 0..ns {
                let u = (i as f64 + random::<f64>()) / nx as f64;
                let v = (j as f64 + random::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                let _p = r.point_at_parameter(2.0);
                col = col + color(&r, &world, 0);
            }

            col = col / (ns as f64);
            col = Vec3{e: [col.r().sqrt(), col.g().sqrt(), col.b().sqrt()]} ;

            let ir = (255.99 * col.r()) as i64;
            let ig = (255.99 * col.g()) as i64;
            let ib = (255.99 * col.b()) as i64;

            write!(buffer, "{} {} {}\n", ir, ig, ib);
        }
    }
    Ok(())
}

fn main() {
    //random_in_unit_sphere();
    let _x = render_ppm().unwrap();
}
