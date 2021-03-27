use crate::visualizations;

/// Represents a visualization that can be serialized into a json string.
pub trait Visualization {
    // TODO: Should be really return a String here?
    // Maybe some JSON type would be better.
    // This would make serde a public dependency though.
    /// Serializes this visualization to JSON.
    /// The JSON should match [this schema](https://hediet.github.io/visualization/docs/visualization-data-schema.json).
    /// There is a playground [here](https://hediet.github.io/visualization/?darkTheme=1).
    fn to_json(&self) -> String;
}

/// Represents something that can provide a visualization for itself.
pub trait Visualizable {
    type V: Visualization;

    /// Returns a suited visualization.
    fn visualize(&self) -> Self::V;
}

impl<T: Visualizable> Visualization for T {
    fn to_json(&self) -> String {
        self.visualize().to_json()
    }
}

impl<'t> Visualizable for &'t str {
    type V = visualizations::Text<'t>;

    fn visualize(&self) -> Self::V {
        visualizations::Text::new((*self).into())
    }
}
