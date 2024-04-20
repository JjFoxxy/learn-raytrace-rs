use std::{fs::File, io::Write};

pub mod geometry;
pub mod interval;
pub mod ray;
pub mod vec3;

use interval::Interval;
use ray::Ray;
use vec3::*;

use crate::geometry::*;

fn write_color(file: &mut File, color: Color) {
    let ir = (color.x * 255.999) as u32;
    let ig = (color.y * 255.999) as u32;
    let ib = (color.z * 255.999) as u32;

    file.write_fmt(format_args!("{ir} {ig} {ib}\n")).unwrap();
}

fn ray_color(ray: &mut ray::Ray, world: &mut HittableList) -> Color {
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

fn render_to_file(filename: &str, world: &mut HittableList) {
    // Image
    let aspect_ratio: f32 = 16. / 9.;
    let image_width: u32 = 400;
    let mut image_height: u32 = ((image_width as f32) / aspect_ratio) as u32;
    if image_height == 0 {
        image_height = 1;
    }

    // Camera
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.;
    let viewport_width: f32 = viewport_height * ((image_width as f32) / (image_height as f32));
    let camera_center = Point3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

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

    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);

    let viewport_upper_left = camera_center
        - Vec3 {
            x: 0.,
            y: 0.,
            z: focal_length,
        }
        - viewport_u / 2.
        - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let open_result = File::create(filename);
    println!("Rendering to the file {filename}");
    match open_result {
        Ok(mut file) => {
            file.write("P3\n".as_bytes()).unwrap();
            file.write_fmt(format_args!("{image_width} {image_height}\n"))
                .unwrap();
            file.write("255\n".as_bytes()).unwrap();

            for i in 0..image_height {
                let remaining = image_height - i;
                println!("Scanlines remaining: {remaining}");
                for j in 0..image_width {
                    let pixel_center =
                        pixel00_loc + (j as f32 * pixel_delta_u) + (i as f32 * pixel_delta_v);
                    let ray_direction = pixel_center - camera_center;
                    let mut ray = Ray {
                        orig: camera_center,
                        dir: ray_direction,
                    };

                    write_color(&mut file, ray_color(&mut ray, world));
                }
            }
        }
        Err(e) => {
            println!("Unable to open file for rendering: {e}");
        }
    }
    println!("Done!");
}

fn main() {
    let mut world = HittableList::new();
    // Lets start with copies
    world.add(Sphere {
        center: Point3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        radius: 0.5,
    });
    world.add(Sphere {
        center: Point3 {
            x: 0.,
            y: -100.5,
            z: -1.,
        },
        radius: 100.,
    });

    render_to_file("image.ppm", &mut world);
}
