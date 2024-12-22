use crate::vec3::Color;

use crate::geometry::HitRecord;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::Vec3;

pub trait Material {
    fn scatter(&self, _ray: &Ray, _record: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: Interval::new(0., 1.).clamp(fuzz),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        let scattered = Ray {
            orig: record.point,
            dir: scatter_direction,
        };
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&ray.dir, &record.normal).unit_vector()
            + self.fuzz * Vec3::random_unit_vector();
        let scattered = Ray {
            orig: record.point,
            dir: reflected,
        };
        let attenuation = self.albedo;
        if scattered.dir.dot(&record.normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
