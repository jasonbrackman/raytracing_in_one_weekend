use std::io::{Write, Result};
use std::fs::File;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn x(self) -> f64 {self.e[0]}
    pub fn y(self) -> f64 {self.e[1]}
    pub fn z(self) -> f64 {self.e[2]}

    pub fn r(self) -> f64 {self.e[0]}
    pub fn g(self) -> f64 {self.e[1]}
    pub fn b(self) -> f64 {self.e[2]}

}

impl Vec3 {
    pub fn new(v:[f64; 3]) -> Self {
        Vec3{e: v}
    }

    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn mul_by_float(self, t: f64) -> Vec3 {
        Vec3{e:[t * self.e[0], t * self.e[1], t * self.e[2]]}
    }

    pub fn div_by_float(self, t: f64) -> Vec3 {
        Vec3{e:[ self.e[0]/t, self.e[1] / t, self.e[2] / t]}
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs:Vec3) -> Self {
        Vec3{e: [self.e[0] + rhs.e[0],
                 self.e[1] + rhs.e[1],
                 self.e[2] + rhs.e[2]]}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs:Vec3) -> Self {
        Vec3{e: [self.e[0] - rhs.e[0],
                 self.e[1] - rhs.e[1],
                 self.e[2] - rhs.e[2]]}
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs:Vec3) -> Self {
        Vec3{e: [self.e[0] * rhs.e[0],
                 self.e[1] * rhs.e[1],
                 self.e[2] * rhs.e[2]]}
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs:Vec3) -> Self {
        Vec3{e: [self.e[0] / rhs.e[0],
                 self.e[1] / rhs.e[1],
                 self.e[2] / rhs.e[2]]}
    }
}



struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {
    fn new(a_: Vec3, b_: Vec3) -> Self { Ray{a: a_, b: b_} }

    pub fn origin(self) -> Vec3 {
        self.a
    }

    pub fn direction(self) -> Vec3 {
        self.b
    }

    pub fn point_of_parameter(self, t: f64) -> Vec3 {
        self.a + self.b.mul_by_float(t)
    }
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

fn unit_vector(v: Vec3) -> Vec3 {
    let length = v.length();
    v.div_by_float(length)
}

fn color(r: Ray) -> Vec3 {
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3{e:[1.0, 1.0, 1.0]}.mul_by_float(1.0 - t) + Vec3{e:[0.5, 0.7, 1.0]}.mul_by_float(t)
}

fn main() {
    let _x = ppm_example().unwrap();
}
