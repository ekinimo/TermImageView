use image::{imageops::FilterType::Lanczos3, GenericImageView};
use terminal_size::{terminal_size, Height, Width};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("supply an image ... ");
        return;
    }
    let mut it = args.iter();
    it.next();
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        println!("Your terminal is {} cols wide and {} lines tall", w, h);
        let width = (w / 2) as u32;
        let height = h as u32;
        for impath in it {
            let img = image::open(impath).unwrap();

            
            let im = img.resize(width, height, Lanczos3).into_rgb8();
            
            im.rows().for_each(|pixels| {
                pixels.for_each(|pixel| {
                    let [red, green, blue] = pixel.0;
                    print!("\x1b[48;2;{red};{green};{blue}m  \x1b[0m");
                });
                println!(" ");
            });
        }
    } else {
        println!("Unable to get terminal size");
    }
}
