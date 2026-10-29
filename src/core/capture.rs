use xcap::{
    Monitor,
    image::{ImageBuffer, Rgba},
};

pub fn capture() -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    let monitors = Monitor::all()?;
    let image = monitors[0].capture_image()?;

    Ok(image)
}
