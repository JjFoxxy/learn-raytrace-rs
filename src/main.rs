use std::{fs::File, io::Write};

fn render_to_file(image_width: u32, image_height: u32, filename: &str) {
    let open_result = File::create(filename);
    println!("Rendering to the file {filename}");
    match open_result {
        Ok(mut file) => {
            file.write("P3\n".as_bytes()).unwrap();
            file.write_fmt(format_args!("{image_width} {image_height}\n")).unwrap();
            file.write("255\n".as_bytes()).unwrap();

            for i in 0..image_height {
                let remaining = image_height - i;
                println!("Scanlines remaining: {remaining}");
                for j in 0..image_width {
                    let r: f32 = i as f32 / (image_width as f32 - 1.);
                    let g: f32 = j as f32 / (image_height as f32 - 1.);
                    let b: f32 = 0.;

                    let ir = (r * 255.999) as u32;
                    let ig = (g * 255.999) as u32;
                    let ib = (b * 255.999) as u32;

                    file.write_fmt(format_args!("{ir} {ig} {ib}\n")).unwrap();
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
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    render_to_file(image_width, image_height, "image.ppm");
}
