use image::*;
use visualizer::visualizations;

/// Exports the visualizer crate.
pub use visualizer;

/// Implements the `visualizer` function for dynamic images.
pub trait VisualizableImage {
    /// Visualizes the given dynamic image as png image.
    fn visualize(&self) -> visualizations::PngImage<'static>;
}

impl VisualizableImage for DynamicImage {
    fn visualize(&self) -> visualizations::PngImage<'static> {
        let mut bytes: Vec<u8> = Vec::new();
        self.write_to(&mut bytes, image::ImageOutputFormat::Png)
            .unwrap();
        visualizations::PngImage::new(&bytes)
    }
}

impl VisualizableImage for GrayImage {
    fn visualize(&self) -> visualizations::PngImage<'static> {
        let i = DynamicImage::ImageLuma8(self.clone());
        i.visualize()
    }
}

impl VisualizableImage for GrayAlphaImage {
    fn visualize(&self) -> visualizations::PngImage<'static> {
        let i = DynamicImage::ImageLumaA8(self.clone());
        i.visualize()
    }
}

impl VisualizableImage for RgbImage {
    fn visualize(&self) -> visualizations::PngImage<'static> {
        let i = DynamicImage::ImageRgb8(self.clone());
        i.visualize()
    }
}

impl VisualizableImage for RgbaImage {
    fn visualize(&self) -> visualizations::PngImage<'static> {
        let i = DynamicImage::ImageRgba8(self.clone());
        i.visualize()
    }
}
