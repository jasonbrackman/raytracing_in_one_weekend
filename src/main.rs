use std::io::{Write, Result};
use std::fs::File;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

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

fn color(r: &Ray) -> Vec3 {
    let t =  hit_sphere(Vec3{e:[0.0, 0.0, -1.0]}, 0.5, r);
    if t > 0.0 {
        let n = vec3::unit_vector(r.point_of_parameter(t) - Vec3{e:[0.0, 0.0, -1.0]});
        return Vec3{e:[n.x() + 1.0, n.y() + 1.0 , n.z() + 1.0]}.mul_by_float(0.5);
    }

    let unit_direction = vec3::unit_vector(*r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3{e:[1.0, 1.0, 1.0]}.mul_by_float(1.0 - t) + Vec3{e:[0.5, 0.7, 1.0]}.mul_by_float(t)
}

fn render_ppm() -> Result<()> {

    let mut buffer = File::create("helloworld.ppm")?;


    let nx = 200;
    let ny = 100;

    write!(buffer, "P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Vec3{e:[-2.0, -1.0, -1.0]};
    let horizontal = Vec3{e:[4.0, 0.0, 0.0]};
    let vertical = Vec3{e:[0.0, 2.0, 0.0]};
    let origin = Vec3{e:[0.0, 0.0, 0.0]};

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;

            let r = Ray::new(origin,  lower_left_corner +
                                                    horizontal.mul_by_float(u) +
                                                    vertical.mul_by_float(v));

            let col = color(&r);

            let ir = (255.99 * col.e[0]) as i64;
            let ig = (255.99 * col.e[1]) as i64;
            let ib = (255.99 * col.e[2]) as i64;

            write!(buffer, "{} {} {}\n", ir, ig, ib);
        }
    }
    Ok(())
}

fn main() {
    let _x = render_ppm().unwrap();
}
