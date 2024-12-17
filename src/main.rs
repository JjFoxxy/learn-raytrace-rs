pub mod camera;
pub mod geometry;
pub mod interval;
pub mod ray;
pub mod vec3;

use camera::Camera;
use vec3::*;

use geometry::*;

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

    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let mut camera = Camera::new(aspect_ratio, image_width);

    camera.render_to_file("image.png", &world);
}
