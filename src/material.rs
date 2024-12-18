use crate::vec3::Color;

use crate::Vec3;
use crate::ray::Ray;
use crate::geometry::HitRecord;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        return None;
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

pub struct Metal {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
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
        let reflected = Vec3::reflect(&ray.dir, &record.normal);
        let scattered = Ray {
            orig: record.point,
            dir: reflected,
        };
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}