pub mod camera;
pub mod geometry;
pub mod interval;
pub mod ray;
pub mod vec3;
pub mod material;

use std::rc::Rc;

use camera::Camera;
use vec3::*;
use material::*;

use geometry::*;

fn main() {
    let mut world = HittableList::new();

    // Prepare materials
    let material_ground = Lambertian {
        albedo: Color {
            x: 0.8,
            y: 0.8,
            z: 0.,
        },
    };
    let material_center = Lambertian {
        albedo: Color {
            x: 0.1,
            y: 0.2,
            z: 0.5,
        },
    };
    let material_left = Metal {
        albedo: Color {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
    };
    let material_right = Metal {
        albedo: Color {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
    };
    // Lets start with copies
    world.add(Sphere {
        center: Point3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        radius: 0.5,
        material: Rc::new(material_center),
    });
    world.add(Sphere {
        center: Point3 {
            x: 0.,
            y: -100.5,
            z: -1.,
        },
        radius: 100.,
        material: Rc::new(material_ground),
    });
    world.add(Sphere {
        center: Point3 {
            x: -1.,
            y: 0.,
            z: -1.,
        },
        radius: 0.5,
        material: Rc::new(material_left),
    });
    world.add(Sphere {
        center: Point3 {
            x: 1.,
            y: 0.,
            z: -1.,
        },
        radius: 0.5,
        material: Rc::new(material_right),
    });

    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let mut camera = Camera::new(aspect_ratio, image_width);

    camera.render_to_file("image.png", &world);
}
