mod visualizable;
pub mod visualizations;

use std::{
    io::Write,
    process::{Command, Stdio},
};
pub use visualizable::{Visualizable, Visualization};

pub use serde;

/// Shows the given visualization in a window.
#[macro_export]
macro_rules! view {
    ($l:expr) => {
        let current_line = line!();
        let path = module_path!();
        let id = format!("{}:{}", path, current_line);
        visualizer::view($l, visualizer::ViewOptions::default().with_id(id));
    };
}

#[derive(Default)]
pub struct ViewOptions {
    id: Option<String>,
    name: Option<String>,
}

impl ViewOptions {
    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

pub fn view(data: &impl Visualization, _options: ViewOptions) {
    let program = "visualize";
    match Command::new(program)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
    {
        Err(error) => {
            eprintln!(
                "Warning: Could not launch `{}`! {} Run `cargo install {}` to install it.",
                program, error, program
            );
        }
        Ok(mut child) => {
            let stdin = child.stdin.as_mut().unwrap();
            writeln!(stdin, "{}", &data.json_data()).unwrap();
            drop(stdin);
            child.wait_with_output().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
