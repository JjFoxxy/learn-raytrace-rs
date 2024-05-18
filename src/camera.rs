use crate::interval::Interval;
use crate::vec3::*;
use crate::{geometry::HittableList, ray::Ray};
use std::{fs::File, io::Write};

#[derive(Default)]
pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    camera_center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Point3,
    pixel_delta_v: Point3,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel: 100,
            max_depth: 50,
            ..Default::default()
        }
    }

    fn write_color(file: &mut File, color: Color) {
        let intensity = Interval::new(0., 0.999);
        let ir = (256. * intensity.clamp(color.x)) as u32;
        let ig = (256. * intensity.clamp(color.y)) as u32;
        let ib = (256. * intensity.clamp(color.z)) as u32;

        file.write_fmt(format_args!("{ir} {ig} {ib}\n")).unwrap();
    }

    pub fn render_to_file(&mut self, filename: &str, world: &HittableList) {
        self.initialize();

        println!("Rendering to the file {filename}");

        let open_result = File::create(filename);

        match open_result {
            Ok(mut file) => {
                file.write_all("P3\n".as_bytes()).unwrap();
                file.write_fmt(format_args!("{} {}\n", self.image_width, self.image_height))
                    .unwrap();
                file.write_all("255\n".as_bytes()).unwrap();

                for j in 0..self.image_height {
                    let remaining = self.image_height - j;
                    println!("Scanlines remaining: {remaining}");
                    for i in 0..self.image_width {
                        let mut pixel_color = Color::default();
                        for _ in 0..self.samples_per_pixel {
                            let mut ray = self.get_ray_from_pixel_position(i, j);
                            pixel_color += Self::ray_color(&mut ray, self.max_depth, world);
                        }
                        Self::write_color(&mut file, self.pixel_samples_scale * pixel_color);
                    }
                }
            }
            Err(e) => {
                println!("Unable to open file for rendering: {e}");
            }
        }
        println!("Done!");
    }

    pub fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as u32;
        if self.image_height == 0 {
            self.image_height = 1;
        }

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.camera_center = Point3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };

        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.;
        let viewport_width: f64 =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        // Horizontal vector
        let viewport_u = Vec3 {
            x: viewport_width,
            y: 0.,
            z: 0.,
        };
        let viewport_v = Vec3 {
            x: 0.,
            y: -viewport_height,
            z: 0.,
        };

        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left = self.camera_center
            - Vec3 {
                x: 0.,
                y: 0.,
                z: focal_length,
            }
            - viewport_u / 2.
            - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(ray: &mut Ray, depth: u32, world: &HittableList) -> Color {
        // When too many colisions - return black
        if depth == 0 {
            return Color::default();
        }

        if let Some(record) = world.hit(
            ray,
            &Interval {
                min: 0.,
                max: f64::INFINITY,
            },
        ) {
            let direction = Vec3::random_on_hemisphere(&record.normal);
            return 0.5
                * Self::ray_color(
                    &mut Ray {
                        orig: record.point,
                        dir: direction,
                    },
                    depth - 1,
                    world,
                );
        }

        let unit_direction = ray.dir.unit_vector();
        let coeff = 0.5 * (unit_direction.y + 1.0);
        (1.0 - coeff)
            * Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            + coeff
                * Color {
                    x: 0.5,
                    y: 0.7,
                    z: 1.0,
                }
    }

    fn sample_square() -> Vec3 {
        Vec3 {
            x: rand::random::<f64>() - 0.5,
            y: rand::random::<f64>() - 0.5,
            z: 0.,
        }
    }

    fn get_ray_from_pixel_position(&self, x: u32, y: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset.x) * self.pixel_delta_u)
            + ((y as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - ray_origin;

        Ray {
            orig: ray_origin,
            dir: ray_direction,
        }
    }
}
