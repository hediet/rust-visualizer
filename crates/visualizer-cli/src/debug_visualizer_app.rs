use std::{borrow::BorrowMut, cell::RefCell, fs::{self, File}, io::Read, path::PathBuf, str::FromStr, sync::{Arc, Mutex}};
use fs::remove_dir_all;
use sha2::{Digest, Sha256};

use futures::{
    channel::oneshot::{self},
    future::Shared,
    FutureExt,
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use serde_json::from_value;
use url::Url;
use wry::{Application, ApplicationProxy, Attributes, CustomProtocol, RpcRequest, WindowProxy};

pub struct DebugVisualizerApp {
    app: Application,
    dist_dir: PathBuf,
}

const BUNDLE_ZIP: &'static [u8] = include_bytes!("../web/dist/bundle.zip");

fn hash(value: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value);
    let result = hasher.finalize();
    let hash_str = format!("{:x}", result);
    return hash_str;
}

impl DebugVisualizerApp {
    pub fn new(cache_dir: PathBuf) -> wry::Result<DebugVisualizerApp> {
        let app = Application::new()?;

        let bundle_hash = hash(BUNDLE_ZIP);
        let dist_dir = cache_dir.join(format!("bundle_{}", bundle_hash));
        if !dist_dir.exists() {
            if cache_dir.exists() {
                let is_safe_to_delete = cache_dir.read_dir().unwrap().all(|e| e.map(|v| v.file_name().to_string_lossy().starts_with("bundle_")).unwrap_or(false));
                if is_safe_to_delete {
                    std::fs::remove_dir_all(cache_dir).expect("Delete entire cache directory");
                } else {
                    eprintln!("Could not delete cache as it contains unexpected files");
                }
            }

            let c = std::io::Cursor::new(BUNDLE_ZIP);
            let mut archive = zip::ZipArchive::new(c).unwrap();
            archive.extract(dist_dir.clone()).expect("Extraction to work");
        }
        Ok(DebugVisualizerApp { app, dist_dir })
    }

    pub fn proxy(&self) -> DebugVisualizerAppProxy {
        DebugVisualizerAppProxy {
            app_proxy: self.app.application_proxy(),
            dist_dir: self.dist_dir.clone()
        }
    }

    pub fn run(self) {
        self.app.run();
    }
}

pub struct DebugVisualizerAppProxy {
    app_proxy: ApplicationProxy,
    dist_dir: PathBuf,
}

#[derive(Deserialize, Serialize)]
struct InitializedEvent {}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum Event {
    Initialized(InitializedEvent),
}


impl DebugVisualizerAppProxy {
    pub fn new_window(&self) -> wry::Result<DebugVisualizerWindow> {
        let attributes = Attributes {
            url: 
                //Some("http://localhost:8080".to_string()), 
                Some("app://host/index.html".to_string()),
            title: String::from("Visualization"),
            //visible: false,
            initialization_scripts: vec![
                String::from("window.sendMessage = function(message) { window.rpc.call('handleMessage', message) };"),
            ],
            ..Default::default()
        };

        let (set_initialized, initialized) = oneshot::channel::<()>();

        let set_initialized = Arc::new(Mutex::new(RefCell::new(Some(set_initialized))));

        let handler = Box::new(move |proxy: WindowProxy, req: RpcRequest| {
            if req.method == "handleMessage" {
                let params = req.params.unwrap();
                let params = params.as_array().unwrap();

                let e: Event = from_value(params.into_iter().next().unwrap().clone()).unwrap();
                match e {
                    Event::Initialized(_) => {
                        set_initialized
                            .lock()
                            .unwrap()
                            .borrow_mut()
                            .take()
                            .unwrap()
                            .send(())
                            .unwrap();
                        proxy.show().unwrap();
                    }
                }
            }
            None
        });

        let base = self.dist_dir.clone();

        let window = self.app_proxy.add_window_with_configs(
            attributes,
            Some(handler),
            Some(CustomProtocol {
                name: String::from("app"),
                handler: Box::new(move |url: &str| {
                    let url = Url::parse(url)?;
                    let path = &url.path()[1..];
                                        
                    let f = base.join(path);
                    
                    if let Ok(mut file) = File::open(f) {
                        let mut buf = Vec::new();
                        file.read_to_end(&mut buf).unwrap();
                        Ok(buf)
                    } else {
                        Err(wry::Error::MessageSender)
                    }
                    
                }),
            }),
            None,
        )?;

        Ok(DebugVisualizerWindow {
            window,
            initialized: initialized.shared(),
        })
    }
}

pub struct DebugVisualizerWindow {
    window: WindowProxy,
    initialized: Shared<oneshot::Receiver<()>>,
}

#[derive(Deserialize, Serialize)]
struct ShowVisualization {
    data: serde_json::Value,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum Message {
    ShowVisualization(ShowVisualization),
}

impl DebugVisualizerWindow {
    async fn send_message(&self, message: Message) {
        let message_str = serde_json::to_string(&message).unwrap();

        self.initialized.clone().await.unwrap();

        self.window
            .evaluate_script(format!("window.processEvent({})", message_str))
            .unwrap();
    }

    pub async fn show_visualization_data(&self, visualization_data: &str) -> wry::Result<()> {
        self.send_message(Message::ShowVisualization(ShowVisualization {
            data: serde_json::from_str(visualization_data).unwrap(),
        }))
        .await;
        Ok(())
    }
}
