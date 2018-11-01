use std::io::{Write, Result};
use std::fs::File;

fn ppm_example() -> Result<()> {

    let mut buffer = File::create("helloworld.ppm")?;


    /* Graphics Hello World!*/
    let nx = 200;
    let ny = 100;

    write!(buffer, "P3\n{} {}\n255\n", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let ir = (255.99 * (i as f64 / nx as f64)) as i64;
            let ig = (255.99 * (j as f64 / ny as f64)) as i64;
            let ib = (255.99 * 0.2) as i64;

            write!(buffer, "{} {} {}\n", ir, ig, ib);
        }
    }
    Ok(())
}

fn main() {
    ppm_example();
}
