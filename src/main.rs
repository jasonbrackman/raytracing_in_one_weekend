use std::io::{Write, Result};
use std::fs::File;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

fn color(r: Ray) -> Vec3 {
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3{e:[1.0, 1.0, 1.0]}.mul_by_float(1.0 - t) + Vec3{e:[0.5, 0.7, 1.0]}.mul_by_float(t)
}

fn ppm_example() -> Result<()> {

    let mut buffer = File::create("helloworld.ppm")?;


    /* Graphics Hello World!*/
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

            let col = color(r);
            // let col = Vec3::new([i as f64 / nx as f64, j as f64 / ny as f64, 0.2]);

            let ir = (255.99 * col.e[0]) as i64;
            let ig = (255.99 * col.e[1]) as i64;
            let ib = (255.99 * col.e[2]) as i64;

            write!(buffer, "{} {} {}\n", ir, ig, ib);
        }
    }
    Ok(())
}

fn main() {
    let _x = ppm_example().unwrap();
}
