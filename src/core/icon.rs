use image::ImageReader;
use std::io::Cursor;

static ICON_PNG: &[u8] = include_bytes!("../../assets/logo-128.png");

pub fn load_icon_rgba() -> (Vec<u8>, u32, u32) {
    let img = ImageReader::new(Cursor::new(ICON_PNG))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();
    let (w, h) = img.dimensions();
    (img.into_raw(), w, h)
}
