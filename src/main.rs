use std::env;
use show_image::{ImageView, ImageInfo, create_window, WindowOptions};
use image::{DynamicImage, open}; 

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("usage: {} <image_path>", args[0]);
        std::process::exit(1);
    }
    
    let path = &args[1];

    let img_buffer = read_image(path);

    let info = ImageInfo::rgba8(img_buffer.width(), img_buffer.height());
    let image_view = ImageView::new(info, img_buffer.as_bytes());



    let options = WindowOptions {
        size: Some([img_buffer.width(), img_buffer.height()]), 
        preserve_aspect_ratio: true, 
        ..Default::default()
    };

    let window = create_window("iv", options)?;
    window.set_image("iv", image_view)?;

    window.wait_until_destroyed()?;
    Ok(())
}

fn read_image(path: &str) -> DynamicImage {
    let on_top = open(path).expect("Error: file not found or not valid").into_rgba8();

    DynamicImage::ImageRgba8(on_top)
}
