use image::{io::Reader as ImageReader, DynamicImage};
use visualizer::{view, visualizations};

fn main() -> std::io::Result<()> {
    let img = ImageReader::open("data/img.png")?.decode().unwrap();
    view!(&img.visualize());
    view!(&"hello world");

    Ok(())
}

trait VisualizableDynamicImage<'t> {
    fn visualize(&'t self) -> visualizations::PngImage<'t>;
}

impl<'t> VisualizableDynamicImage<'t> for DynamicImage {
    fn visualize(&'t self) -> visualizations::PngImage<'t> {
        let mut bytes: Vec<u8> = Vec::new();
        self.write_to(&mut bytes, image::ImageOutputFormat::Png)
            .unwrap();

        visualizations::PngImage::new(&bytes)
    }
}
