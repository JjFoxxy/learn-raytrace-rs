pub mod vec3;

use vec3::Vec3;

type Point3 = Vec3;

struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Self {
        self.orig + self.dir * t
    }
}