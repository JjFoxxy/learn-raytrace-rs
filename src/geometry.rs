use crate::ray::Ray;
use crate::vec3::*;

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
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

pub trait Hittable {
    fn hit(&self, ray: &mut Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>;
}

pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::<Box<dyn Hittable + 'a>>::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: impl Hittable + 'a) {
        self.objects.push(Box::new(object));
    }

    pub fn hit(&self, ray: &mut Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut closest_so_far = ray_tmax;
        let mut result: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, ray_tmin, closest_so_far) {
                closest_so_far = record.t;
                result = Some(record);
            }
        }

        result
    }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &mut Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut record = HitRecord::default();
        let origin_center = ray.orig - self.center;
        let a = ray.dir.len_squared();
        let half_b = origin_center.dot(&ray.dir);
        let c = origin_center.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if (root <= ray_tmin) || (ray_tmax <= root) {
            root = (-half_b + sqrtd) / a;
            if (root <= ray_tmin) || (ray_tmax <= root) {
                return None;
            }
        }

        record.t = root;
        record.point = ray.at(record.t);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);

        return Some(record);
    }
}
