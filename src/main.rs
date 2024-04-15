use std::{fs::File, io::Write};

pub mod ray;
pub mod vec3;

use vec3::Vec3;

use crate::ray::Ray;

type Color = Vec3;
type Point3 = Vec3;

fn write_color(file: &mut File, color: Color) {
    let ir = (color.x * 255.999) as u32;
    let ig = (color.y * 255.999) as u32;
    let ib = (color.z * 255.999) as u32;

    file.write_fmt(format_args!("{ir} {ig} {ib}\n")).unwrap();
}

fn ray_color(ray: &ray::Ray) -> Color {
    let t = hit_sphere(
        Point3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        0.5,
        ray,
    );
    if t > 0. {
        let n = (ray.at(t)
            - Vec3 {
                x: 0.,
                y: 0.,
                z: -1.,
            })
        .unit_vector();
        return 0.5
            * Color {
                x: n.x + 1.,
                y: n.y + 1.,
                z: n.z + 1.,
            };
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

fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> f32 {
    let origin_center = ray.orig - center;
    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * origin_center.dot(ray.dir);
    let c = origin_center.dot(origin_center) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
        -1.
    } else {
        (-b - discriminant.sqrt()) / (2. * a)
    }
}

fn render_to_file(filename: &str) {
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
                    let ray = Ray {
                        orig: camera_center,
                        dir: ray_direction,
                    };

                    write_color(&mut file, ray_color(&ray));
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
    render_to_file("image.ppm");
}
