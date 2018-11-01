use std::io::{Write, Result};
use std::fs::File;
use std::ops::{Add, Mul};


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
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs:Vec3) -> Self {
        Vec3{e: [self.e[0] + rhs.e[0],
                 self.e[1] + rhs.e[1],
                 self.e[2] + rhs.e[2]]}
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
            let ir = (255.99 * (i as f64 / nx as f64)) as i64;
            let ig = (255.99 * (j as f64 / ny as f64)) as i64;
            let ib = (255.99 * 0.2) as i64;

            write!(buffer, "{} {} {}\n", ir, ig, ib);
        }
    }
    Ok(())
}

fn main() {
    ppm_example();
}
