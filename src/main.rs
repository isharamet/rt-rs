use std::fs::File;
use std::io::Write;

fn main() {
    let img_format = "P3";
    let img_width: u32 = 256;
    let img_height: u32 = 256;
    let max_colors: u32 = 255;

    let mut file = File::create("img.ppm").expect("Unable to create file");

    write!(
        file,
        "{}\n{} {}\n{}\n",
        img_format, img_width, img_height, max_colors
    )
    .expect("Unable to write to file");

    for i in 0..img_height {
        print!("\rScanlines remaining: {}", img_height - i);
        for j in 0..img_width {
            let r = j as f32 / (img_width - 1) as f32;
            let g = i as f32 / (img_height - 1) as f32;
            let b: f32 = 0.0;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            write!(file, "{} {} {}\n", ir, ig, ib).expect("Unable to write to file");
        }
    }
}
