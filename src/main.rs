use anyhow::Context;
use image::io::Reader;
use std::env;

const SYMBOLS: [char; 9] = ['.', ',', '_', '-', '"', '*', '•', '=', '@'];

fn main() -> Result<(), anyhow::Error> {
    let divisor = (1020_f32 / SYMBOLS.len() as f32).ceil() as u32;
    let mut args = env::args().skip(1);
    let filename = match args.next() {
        Some(filename) => filename,
        None => {
            anyhow::bail!("Usage: ascii <image-path> [zoom-in-scale=1]");
        }
    };
    let scale: u32 = args
        .next()
        .unwrap_or_else(|| "1".to_string())
        .parse()
        .unwrap();

    if scale == 0 {
        anyhow::bail!("Scale cannot be 0");
    }

    let img = Reader::open(filename)
        .context("Could not open file")?
        .decode()
        .context("Could not decode file")?
        .to_rgba8();

    for y in 0..(img.height() / (scale * 2)) {
        for x in 0..(img.width() / scale) {
            let mut opacity = 0;
            let mut value: u32 = 0;
            for i in 0..scale {
                for j in 0..(scale * 2) {
                    let p = img.get_pixel(x * scale + i, y * scale * 2 + j);
                    opacity += p.0[3] as u32;
                    value += p.0[0] as u32 + p.0[1] as u32 + p.0[2] as u32;
                }
            }
            if opacity < scale * 64 {
                print!(" ");
            } else {
                let symbol = ((value as f32 / (scale as f32 * scale as f32 * 2.0)).floor()
                    / divisor as f32)
                    .floor();
                print!("{}", SYMBOLS[symbol as usize]);
            }
        }
        println!("");
    }

    Ok(())
}
