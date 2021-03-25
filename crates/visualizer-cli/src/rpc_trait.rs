use jsonrpc_core::{Result, Value};
use jsonrpc_derive::rpc;
use serde_json::value::RawValue;

#[rpc]
pub trait VisualizerRpcServer {
    #[rpc(name = "showVisualization")]
    fn show_visualization(&self, data: Box<RawValue>) -> Result<()>;
}
// {"jsonrpc": "2.0", "method": "showVisualization", "params": [{ "kind": { "graph": true }, "nodes": [  { "id": "2" } ], "edges": [ { "from": "1", "to": "2", "color": "red" }, { "from": "1", "to": "3" } ]}], "id": 3}
