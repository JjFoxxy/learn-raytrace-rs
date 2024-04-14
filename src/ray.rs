use crate::vec3::Vec3;

type Point3 = Vec3;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + self.dir * t
    }
}