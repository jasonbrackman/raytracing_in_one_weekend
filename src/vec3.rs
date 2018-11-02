use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
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

pub fn unit_vector(v: Vec3) -> Vec3 {
    let length = v.length();
    v.div_by_float(length)
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
}