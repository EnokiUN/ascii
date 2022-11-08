use image::io::Reader;
use std::env;

fn main() {
    let symbols = [
        '.', ',', '_', '\'', '-', '^', '"', ':', ';', '*', '•', '=', '$', '&', '@', '#',
    ];

    let divisor = (1020_f32 / symbols.len() as f32).ceil() as u8;
    let filename = env::args().nth(1).expect("You must specify a file name");
    let scale: usize = env::args()
        .nth(2)
        .unwrap_or_else(|| "1".to_string())
        .parse()
        .unwrap();
    let img = Reader::open(filename).unwrap().decode().unwrap().to_rgba8();

    img.rows().enumerate().for_each(|(i, p)| {
        if i % scale * 2 == 0 {
            p.enumerate().for_each(|(i, p)| {
                if i % scale == 0 {
                    let rgba = p.0;
                    if rgba[3] == 0 {
                        print!(" ");
                    } else {
                        let opacity =
                            rgba[0] as u16 + rgba[1] as u16 + rgba[2] as u16 + rgba[3] as u16;
                        let value = opacity / divisor as u16;
                        print!("{}", symbols[value as usize]);
                    }
                }
            });
            println!("");
        }
    });
}
