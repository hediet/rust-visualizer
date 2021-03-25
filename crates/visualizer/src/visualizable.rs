use crate::visualizations;
pub trait Visualization {
    fn json_data(&self) -> String;
}

pub trait Visualizable {
    type V: Visualization;

    fn visualization(&self) -> Self::V;
}

impl<T: Visualizable> Visualization for T {
    fn json_data(&self) -> String {
        self.visualization().json_data()
    }
}

impl<'t> Visualizable for &'t str {
    type V = visualizations::Text<'t>;

    fn visualization(&self) -> Self::V {
        visualizations::Text::new((*self).into())
    }
}
