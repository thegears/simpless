use std::borrow::Cow;

use arboard::{Clipboard, ImageData, SetExtLinux};
use xcap::image::{ImageBuffer, Rgba};

pub fn copy(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = Clipboard::new()?;
    let setter = ctx.set();
    let (width, height) = image.dimensions();
    let raw_pixels = image.into_raw();

    setter
        .wait()
        .image(ImageData {
            width: width as usize,
            height: height as usize,
            bytes: Cow::from(raw_pixels),
        })
        .unwrap();

    Ok(())
}
