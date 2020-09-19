mod ray;
mod vec3;

use vec3::Vec3;

fn main() {
    // Image

    const IMG_WIDTH: usize = 256;
    const IMG_HEIGHT: usize = 256;

    // Render

    println!("P3");
    println!("{width} {height}", width = IMG_WIDTH, height = IMG_HEIGHT);
    println!("{color_depth}", color_depth = 255);

    for j in (0..IMG_HEIGHT).rev() {
        let steps = (IMG_HEIGHT - 1) / 10;
        if j % steps == 0 {
            eprint!("\rScanlines remaining: {:>4}", j);
        }
        for i in 0..IMG_WIDTH {
            let color = Vec3::new(
                (i as f64) / (IMG_WIDTH as f64 - 1.0),
                (j as f64) / (IMG_HEIGHT as f64 - 1.0),
                0.25,
            );
            println!("{}", color);
        }
    }

    eprintln!();
    eprintln!("Done.");
}
