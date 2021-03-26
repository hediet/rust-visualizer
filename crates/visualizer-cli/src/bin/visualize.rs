use std::{
    io::{stdin, Read},
    thread,
};
use structopt::StructOpt;
use visualizer_cli::{run_rpc, DebugVisualizerApp};

#[derive(StructOpt, Debug)]
#[structopt(name = "visualize")]
struct Opt {
    #[structopt(long)]
    rpc: bool,
}

fn main() -> wry::Result<()> {
    let opt = Opt::from_args();

    let app = DebugVisualizerApp::new()?;
    let proxy = app.proxy();

    thread::spawn(move || {
        if opt.rpc {
            run_rpc(proxy);
        } else {
            let visualization_data = get_stdin_data().unwrap();

            futures::executor::block_on(async {
                let window = proxy.new_window().unwrap();

                window
                    .show_visualization_data(&visualization_data)
                    .await
                    .unwrap();
            });
        }
    });

    app.run();

    Ok(())
}

fn get_stdin_data() -> wry::Result<String> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;
    Ok(buf)
}
