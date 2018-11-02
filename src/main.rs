use std::io::{Write, Result};
use std::fs::File;
use std::ops::{Add, Sub, Mul, Div};


struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    fn x(self) -> f64 {self.e[0]}
    fn y(self) -> f64 {self.e[1]}
    fn z(self) -> f64 {self.e[2]}

    fn r(self) -> f64 {self.e[0]}
    fn g(self) -> f64 {self.e[1]}
    fn b(self) -> f64 {self.e[2]}

}

impl Vec3 {
    fn new(v:[f64; 3]) -> Self {
        Vec3{e: v}
    }

    fn length(self) -> f64{
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
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

    fn origin(self) -> Vec3 {
        self.a
    }

    fn direction(self) -> Vec3 {
        self.b
    }

    fn point_of_parameter(self, t: f64) -> f64 {
        self.a + t * self.b
    }
}
fn ppm_example() -> Result<()> {

    let mut buffer = File::create("helloworld.ppm")?;


    /* Graphics Hello World!*/
    let nx = 200;
    let ny = 100;

    write!(buffer, "P3\n{} {}\n255\n", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new([i as f64 / nx as f64, j as f64 / ny as f64, 0.2]);
            let ir = (255.99 * col.e[0]) as i64;
            let ig = (255.99 * col.e[1]) as i64;
            let ib = (255.99 * col.e[2]) as i64;

            write!(buffer, "{} {} {}\n", ir, ig, ib);
        }
    }
    Ok(())
}

fn main() {
    ppm_example();
}
