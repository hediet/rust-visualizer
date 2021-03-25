use jsonrpc_core::{
    futures::{self, FutureExt},
    BoxFuture, IoHandler, Result,
};
use jsonrpc_core_client::transports::local;
use jsonrpc_derive::rpc;

#[rpc]
pub trait VisualizerRpcServer {
    #[rpc(name = "showVisualization")]
    fn show_visualization(&self, data: Value) -> Result<()>;
}
