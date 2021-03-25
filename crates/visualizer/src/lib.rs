mod visualizable;
pub mod visualizations;

use std::{
    io::Write,
    process::{Command, Stdio},
};
pub use visualizable::{Visualizable, Visualization};

pub use serde;

#[macro_export]
macro_rules! view {
    ($l:expr) => {
        view($l);
    };
}

pub fn view(data: &impl Visualization) {
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
