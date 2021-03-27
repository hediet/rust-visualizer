# Visualizer

A crate to visualize image instances of the `image` create. Requires the crate `visualizer-cli` to be installed globally.

Based on [@hediet/visualization](https://github.com/hediet/visualization) which also powers
the [Debug Visualizer extension for VS Code](https://github.com/hediet/vscode-debug-visualizer).

## Installation

```
cargo install visualizer-cli
cargo add image-visualizer
```

## Example

Use the `view!` macro to view a visualization.
The `visualize` function creates a visualization for the given image.

```rust
use image::io::Reader as ImageReader;
use image_visualizer::{VisualizableImage, visualizer::view};

fn main() -> std::io::Result<()> {
    let img = ImageReader::open("data/img.png")?.decode().unwrap();
    view!(&img.visualize());
    Ok(())
}
```

![Screenshot](https://github.com/hediet/rust-visualizer/raw/HEAD/crates/image-visualizer/docs/screenshot.png)
