mod visualizable;
pub mod visualizations;

use std::{
    io::Write,
    process::{Command, Stdio},
};
pub use visualizable::{Visualizable, Visualization};

pub use serde;

/// Shows the given visualization in a new window.
/// Waits until the visualization window is closed.
#[macro_export]
macro_rules! view {
    ($l:expr) => {
        let current_line = line!();
        let path = module_path!();
        let id = format!("{}:{}", path, current_line);
        visualizer::view(
            $l,
            visualizer::ViewOptions::default()
                .with_source_id(id.clone())
                .with_title(id),
        );
    };
}

/// Configures visualization view options.
#[derive(Default)]
pub struct ViewOptions {
    #[allow(dead_code)]
    source_id: Option<String>,
    title: Option<String>,
}

impl ViewOptions {
    /// Sets a source id.
    // Will be used in the future to detect repeated view calls.
    pub fn with_source_id(mut self, id: String) -> Self {
        self.source_id = Some(id);
        self
    }

    /// Sets the title of the visualizer window.
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
}

/// Shows the given visualization in a window and waits until the window is closed.
/// Prefer using the `view!` instead, as it includes the current filename and line.
pub fn view(data: &impl Visualization, options: ViewOptions) {
    let program = "visualize";
    let mut cmd = Command::new(program);
    cmd.stdin(Stdio::piped()).stdout(Stdio::null());
    if let Some(title) = options.title {
        cmd.arg("--title").arg(title);
    }
    match cmd.spawn() {
        Err(error) => {
            eprintln!(
                "Warning: Could not launch `{}`! {} Run `cargo install {}` to install it.",
                program, error, program
            );
        }
        Ok(mut child) => {
            let stdin = child.stdin.as_mut().unwrap();
            writeln!(stdin, "{}", &data.to_json()).unwrap();
            drop(stdin);
            child.wait_with_output().unwrap();
        }
    }
}
