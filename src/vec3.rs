use std::ops;

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn reflect(v: &Self, n: &Self) -> Self {
        *v - *n * v.dot(n) * 2.
    }

    pub fn near_zero(&self) -> bool {
        const DELTA: f64 = 1e-8;
        self.x.abs() < DELTA && self.y.abs() < DELTA && self.z.abs() < DELTA
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1., 1.);
            if p.len_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let v = Self::random_unit_vector();
        if v.dot(normal) > 0. {
            v
        } else {
            -v
        }
    }

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn random_unit() -> Self {
        Vec3 {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
        }
    }

    pub fn random(min: f64, max: f64) -> Self {
        let scale = max - min;
        Vec3 {
            x: min + scale * rand::random::<f64>(),
            y: min + scale * rand::random::<f64>(),
            z: min + scale * rand::random::<f64>(),
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        let mut res = self;
        res.x = -res.x;
        res.y = -res.y;
        res.z = -res.z;
        res
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut res = self;
        res.x += rhs.x;
        res.y += rhs.y;
        res.z += rhs.z;
        res
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut res = self;
        res.x -= rhs.x;
        res.y -= rhs.y;
        res.z -= rhs.z;
        res
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut res = self;
        res.x *= rhs;
        res.y *= rhs;
        res.z *= rhs;
        res
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let mut res = rhs;
        res.x *= self;
        res.y *= self;
        res.z *= self;
        res
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        if rhs == 0. {
            panic!("Can not divide by 0: Vec3");
        }
        let mut res = self;
        res.x /= rhs;
        res.y /= rhs;
        res.z /= rhs;
        res
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        if rhs == 0. {
            panic!("Can not divide by 0: Vec3");
        }
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn len_success() {
        assert_eq!(
            5.,
            Vec3 {
                x: 4.,
                y: 3.,
                z: 0.
            }
            .len()
        );
    }

    #[test]
    fn unit_success() {
        assert_eq!(
            1.,
            Vec3 {
                x: 6.,
                y: 5.,
                z: 7.
            }
            .unit_vector()
            .len()
        );
    }

    #[test]
    fn add_success() {
        assert_eq!(
            Vec3 {
                x: 3.,
                y: 3.,
                z: 3.
            },
            Vec3 {
                x: 2.,
                y: 2.,
                z: 2.
            } + Vec3 {
                x: 1.,
                y: 1.,
                z: 1.
            }
        );
    }

    #[test]
    fn add_assign_success() {
        let mut vec = Vec3 {
            x: 2.,
            y: 2.,
            z: 2.,
        };
        vec += Vec3 {
            x: 1.,
            y: 1.,
            z: 1.,
        };
        assert_eq!(
            Vec3 {
                x: 3.,
                y: 3.,
                z: 3.
            },
            vec
        );
    }

    #[test]
    fn sub_success() {
        assert_eq!(
            Vec3 {
                x: 3.,
                y: 3.,
                z: 3.
            },
            Vec3 {
                x: 6.,
                y: 5.,
                z: 4.
            } - Vec3 {
                x: 3.,
                y: 2.,
                z: 1.
            }
        );
    }

    #[test]
    fn mul_success() {
        assert_eq!(
            Vec3 {
                x: 18.,
                y: 15.,
                z: 12.
            },
            Vec3 {
                x: 6.,
                y: 5.,
                z: 4.
            } * 3.
        );
    }

    #[test]
    fn mul_left_success() {
        assert_eq!(
            Vec3 {
                x: 18.,
                y: 15.,
                z: 12.
            },
            3. * Vec3 {
                x: 6.,
                y: 5.,
                z: 4.
            }
        );
    }

    #[test]
    fn mul_assign_success() {
        let mut vec = Vec3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        vec *= 4.;
        assert_eq!(
            Vec3 {
                x: 4.,
                y: 8.,
                z: 12.
            },
            vec
        );
    }

    #[test]
    fn div_success() {
        assert_eq!(
            Vec3 {
                x: 2.,
                y: 3.,
                z: 4.
            },
            Vec3 {
                x: 6.,
                y: 9.,
                z: 12.
            } / 3.
        );
    }

    #[test]
    fn div_assign_success() {
        let mut vec = Vec3 {
            x: 5.,
            y: 10.,
            z: 15.,
        };
        vec /= 5.;
        assert_eq!(
            Vec3 {
                x: 1.,
                y: 2.,
                z: 3.
            },
            vec
        );
    }

    #[test]
    fn dot_success() {
        assert_eq!(
            147.,
            Vec3 {
                x: 7.,
                y: 7.,
                z: 7.
            }
            .dot(&Vec3 {
                x: 7.,
                y: 7.,
                z: 7.
            })
        );
    }

    #[test]
    fn cross_success() {
        assert_eq!(
            Vec3 {
                x: -3.,
                y: 6.,
                z: -3.
            },
            Vec3 {
                x: 4.,
                y: 5.,
                z: 6.
            }
            .cross(&Vec3 {
                x: 7.,
                y: 8.,
                z: 9.
            })
        );
    }
}
