use std::cell::RefCell;

use async_std::{sync::Mutex, task::block_on};
use jsonrpc_stdio_server::jsonrpc_core::*;
use jsonrpc_stdio_server::ServerBuilder;
use serde_json::value::RawValue;
use std::boxed::Box;
use tokio;

use crate::{debug_visualizer_app::WindowOptions, DebugVisualizerAppProxy};
use crate::{rpc_trait::VisualizerRpcServer, DebugVisualizerWindow};

struct RpcState {
    app: DebugVisualizerAppProxy,
    window: Option<DebugVisualizerWindow>,
}

struct VisualizerRpcServerImpl {
    state: Mutex<RefCell<RpcState>>,
}

impl VisualizerRpcServer for VisualizerRpcServerImpl {
    fn show_visualization(&self, data: Box<RawValue>) -> Result<()> {
        block_on(async move {
            let m = self.state.lock().await;
            let mut s = m.borrow_mut();
            if s.window.is_none() {
                let w = s.app.new_window(WindowOptions { title: None }).unwrap();
                s.window = Some(w);
            }
            s.window
                .as_ref()
                .unwrap()
                .show_visualization_data(&data.to_string())
                .await
                .unwrap();

            Ok(())
        })
    }
}

impl VisualizerRpcServerImpl {
    pub fn new(app: DebugVisualizerAppProxy) -> Self {
        Self {
            state: Mutex::new(RefCell::new(RpcState { app, window: None })),
        }
    }
}

#[tokio::main]
pub async fn run_rpc(app: DebugVisualizerAppProxy) {
    let mut io = IoHandler::default();
    io.extend_with(VisualizerRpcServerImpl::new(app).to_delegate());

    let server = ServerBuilder::new(io).build();
    server.await;
}
