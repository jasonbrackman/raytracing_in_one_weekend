extern crate rand;
use rand::{random};

use std::time::Instant;

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


fn color(r: &Ray, world: &HitableList, depth: i32) -> Vec3 {
    let max_float = 100_000_000_000.00;
    let rec = &mut HitRecord::new();

    if world.hit(r, 0.001, max_float, rec) {
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
        material: Lambertian::new(0.1, 0.2, 0.8)
    };

    let sphere_02 = Sphere{
        center:Vec3{e:[0.0, -100.5, -1.0]},
        radius:100.0,
        material: Lambertian::new(0.8, 0.8, 0.0)
    };

    let sphere_03 = Sphere{
        center:Vec3{e:[1.0, 0.0, -1.0]},
        radius:0.5,
        material: Metal::new(0.8, 0.6, 0.2, 0.3)
    };

    let sphere_04 = Sphere{
        center:Vec3{e:[-1.0, 0.0, -1.0]},
        radius:0.5,
        material: Dielectric::new(1.5)    // air = 1,
                                                        // glass = 1.3-1.7,
                                                        // diamond = 2.4
    };

    let sphere_05 = Sphere{
        center:Vec3{e:[-1.0, 0.0, -1.0]},
        radius:-0.45,
        material: Dielectric::new(1.5)
    };

    vec!(sphere_01, sphere_02, sphere_03, sphere_04, sphere_05)
}

fn random_scene() -> Vec<Sphere> {

    let mut scene = vec!();

    scene.push(Sphere{
        center:Vec3{e:[0.0, -1000.0, 0.0]},
        radius:1000.0,
        material: Lambertian::new(0.5, 0.5, 0.5)
    });


    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f32>();
            let center = Vec3{e:[a as f32 + 0.9 + random::<f32>(), 0.2, b as f32 + 0.9 + random::<f32>()]};
            if (center - Vec3 { e: [4.0, 0.2, 0.0] }).length() > 0.9 {
                if choose_mat < 0.8 {
                    //
                    scene.push(
                        Sphere {
                            center,
                            radius: 0.2,
                            material: Lambertian::new(random::<f32>() * random::<f32>(), random::<f32>() * random::<f32>(), random::<f32>() * random::<f32>())
                        })
                } else if choose_mat > 0.8 && choose_mat < 0.95 {
                    //
                    scene.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal::new(0.5 * (1.0 + random::<f32>()), 0.5 * (1.0 + random::<f32>()), 0.5 * (1.0 + random::<f32>()), 0.5 * random::<f32>())
                    });
                } else {
                    // glass
                    scene.push(Sphere { center, radius: 0.2, material: Dielectric::new(1.5)})
                };
            }
        }
    }

    scene.push(Sphere{
        center:Vec3{e:[0.0, 1.0, 0.0]},
        radius:1.0,
        material: Dielectric::new(1.5)
    });
    scene.push(Sphere{
        center:Vec3{e:[-4.0, 1.0, 0.0]},
        radius:1.0,
        material: Lambertian::new(0.4, 0.2, 0.1)
    });
    scene.push(Sphere{
        center:Vec3{e:[4.0, 1.0, 0.0]},
        radius:1.0,
        material: Metal::new(0.7, 0.6, 0.5, 0.0)
    });

    scene
}

fn render_ppm(aa_quality: i32) -> Result<()> {
    // Writes out PPM file which is a text based image format. //

    let mut buffer = File::create("helloworld.ppm")?;


    let nx = 800;
    let ny = 400;
    let aa_samples = aa_quality;

    write!(buffer, "P3\n{} {}\n255\n", nx, ny);

    let objects = random_scene(); //get_spheres();

    let world = hitablelist::HitableList{hit_records: &objects};

    let look_from = Vec3{e:[-8.0, 2.0, 2.0]};
    let look_at = Vec3{e:[0.0, 0.0, -1.0]};
    let distance_to_focus = (look_from - look_at).length();
    let aperture = 0.22;

    let cam = &camera::Camera::new(
                                   look_from,
                                   look_at,
                                   Vec3{e:[0.0, 1.0, 0.0]},
                                   30.0,
                                   nx as f32 / ny as f32,
                                    aperture,
                                    distance_to_focus
    );

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new();
            for _s in 0..aa_samples {
                let u = (i as f32 + random::<f32>()) / nx as f32;
                let v = (j as f32 + random::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                //let _p = r.point_at_parameter(2.0);
                col = col + color(&r, &world, 0);
            }

            col = col / (aa_samples as f32);
            col = Vec3{e: [col.r().sqrt(), col.g().sqrt(), col.b().sqrt()]} ;

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            writeln!(buffer, "{} {} {}\n", ir, ig, ib);
        }
    }
    Ok(())
}

fn main() {

    // Marker for benchmarking start
    let start = Instant::now();

    render_ppm(100).unwrap();

    // Benchmarking
    let time = Instant::now() - start;
    let time_secs = time.as_secs();
    let time_millis = time.subsec_millis();

    println!("Rendered in {} seconds.", time_secs as f32 + time_millis as f32 / 1000.0);
}
