use crate::interval::Interval;
use crate::vec3::*;
use crate::{geometry::HittableList, ray::Ray};
use std::{fs::File, io::Write};

#[derive(Default)]
pub struct Camera {
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    camera_center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Point3,
    pixel_delta_v: Point3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        Camera {
            aspect_ratio,
            image_width,
            ..Default::default()
        }
    }

    fn write_color(file: &mut File, color: Color) {
        let ir = (color.x * 255.999) as u32;
        let ig = (color.y * 255.999) as u32;
        let ib = (color.z * 255.999) as u32;

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

                for i in 0..self.image_height {
                    let remaining = self.image_height - i;
                    println!("Scanlines remaining: {remaining}");
                    for j in 0..self.image_width {
                        let pixel_center = self.pixel00_loc
                            + (j as f32 * self.pixel_delta_u)
                            + (i as f32 * self.pixel_delta_v);
                        let ray_direction = pixel_center - self.camera_center;
                        let mut ray = Ray {
                            orig: self.camera_center,
                            dir: ray_direction,
                        };

                        Self::write_color(&mut file, Self::ray_color(&mut ray, world));
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
        self.image_height = ((self.image_width as f32) / self.aspect_ratio) as u32;
        if self.image_height == 0 {
            self.image_height = 1;
        }

        self.camera_center = Point3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };

        let focal_length: f32 = 1.0;
        let viewport_height: f32 = 2.;
        let viewport_width: f32 =
            viewport_height * ((self.image_width as f32) / (self.image_height as f32));

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

        self.pixel_delta_u = viewport_u / (self.image_width as f32);
        self.pixel_delta_v = viewport_v / (self.image_height as f32);

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

    fn ray_color(ray: &mut Ray, world: &HittableList) -> Color {
        if let Some(record) = world.hit(
            ray,
            &Interval {
                min: 0.,
                max: f32::INFINITY,
            },
        ) {
            return 0.5
                * (record.normal
                    + Color {
                        x: 1.,
                        y: 1.,
                        z: 1.,
                    });
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
}
