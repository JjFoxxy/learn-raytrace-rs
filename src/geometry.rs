use crate::vec3::*;
use crate::ray::Ray;

struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f32,
}

trait Hittable {
    fn hit(ray: & Ray, ray_tmin: f32, ray_tmax: f32, record: &mut HitRecord) -> bool;
}

struct Sphere {
    center: Point3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: & Ray, ray_tmin: f32, ray_tmax: f32, record: &mut HitRecord) -> bool {
        let origin_center = ray.orig - self.center;
        let a = ray.dir.len_squared();
        let half_b = origin_center.dot(ray.dir);
        let c = origin_center.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        
        let root = (-half_b - sqrtd) / a;
        if (root <= ray_tmin) || (ray_tmax <= root) {
            root = (-half_b + sqrtd) / a;
            if (root <= ray_tmin) || (ray_tmax <= root) {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(record.t);
        record.normal = (record.point - self.center) / radius;

        return true;
    }
}