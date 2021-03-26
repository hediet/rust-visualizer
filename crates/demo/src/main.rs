use image::io::Reader as ImageReader;
use image_visualizer::VisualizableImage;
use visualizer::{view, visualizations};

fn main() -> std::io::Result<()> {
    view!(&visualizations::Plotly::of_y(&vec![
        1.0, 2.0, 4.0, 9.0, 16.0
    ]));

    let img = ImageReader::open("data/img.png")?.decode().unwrap();
    view!(&img.visualize());

    view!(&"hello world");

    Ok(())
}
