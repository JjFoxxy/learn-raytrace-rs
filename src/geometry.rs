use crate::ray::Ray;
use crate::vec3::*;

struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    // outward_normal must be unit length
    fn set_face_normal(&mut self, ray: &mut Ray, outward_normal: &Vec3) {
        self.front_face = ray.dir.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

trait Hittable {
    fn hit(&self, ray: &mut Ray, ray_tmin: f32, ray_tmax: f32, record: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn clear(&mut self) {
        self.objects.clear();
    }

    fn add(&mut self, object: impl Hittable) {
        todo!()
    }
}

struct Sphere {
    center: Point3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &mut Ray, ray_tmin: f32, ray_tmax: f32, record: &mut HitRecord) -> bool {
        let origin_center = ray.orig - self.center;
        let a = ray.dir.len_squared();
        let half_b = origin_center.dot(&ray.dir);
        let c = origin_center.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if (root <= ray_tmin) || (ray_tmax <= root) {
            root = (-half_b + sqrtd) / a;
            if (root <= ray_tmin) || (ray_tmax <= root) {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(record.t);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);

        return true;
    }
}
