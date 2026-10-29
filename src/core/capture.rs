use xcap::{
    Monitor,
    image::{ImageBuffer, Rgba},
};

pub fn capture() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let monitors = Monitor::all().unwrap();
    let image = monitors[0].capture_image().unwrap();

    image
}
