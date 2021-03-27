# Visualizer

A crate to visualize data. Requires the crate `visualizer-cli` to be installed globally.

Based on [@hediet/visualization](https://github.com/hediet/visualization) which also powers
the [Debug Visualizer extension for VS Code](https://github.com/hediet/vscode-debug-visualizer).

## Installation

```
cargo install visualizer-cli
cargo add visualizer
```

## Example

```rust
use visualizer::{view, visualizations};

fn main() {
    view!(&visualizations::Plotly::of_y(&vec![
        1.0, 2.0, 4.0, 9.0, 16.0
    ]));
}
```

## Creates That Provide Visualizations

-   `image-visualizer`: Provides visualizations for images from the `image` crate.
