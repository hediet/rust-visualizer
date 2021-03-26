use std::{borrow::BorrowMut, cell::RefCell, io::Read,  sync::{Arc, Mutex}};

use futures::{
    channel::oneshot::{self},
    future::Shared,
    FutureExt,
};
use serde::{Deserialize, Serialize};
use serde_json::from_value;
use url::Url;
use wry::{Application, ApplicationProxy, Attributes, CustomProtocol, RpcRequest, WindowProxy};

pub struct DebugVisualizerApp {
    app: Application,
}

const BUNDLE_ZIP: &'static [u8] = include_bytes!("../web/dist/bundle.zip");


impl DebugVisualizerApp {
    pub fn new() -> wry::Result<DebugVisualizerApp> {
        let app = Application::new()?;

        Ok(DebugVisualizerApp { app })
    }

    pub fn proxy(&self) -> DebugVisualizerAppProxy {
        DebugVisualizerAppProxy {
            app_proxy: self.app.application_proxy(),
        }
    }

    pub fn run(self) {
        self.app.run();
    }
}

pub struct DebugVisualizerAppProxy {
    app_proxy: ApplicationProxy,
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

        let c = std::io::Cursor::new(BUNDLE_ZIP);
        let zip = Arc::new(Mutex::new(zip::ZipArchive::new(c).unwrap()));
        

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

        let window = self.app_proxy.add_window_with_configs(
            attributes,
            Some(handler),
            Some(CustomProtocol {
                name: String::from("app"),
                handler: Box::new(move |url: &str| {
                    let url = Url::parse(url)?;
                    let path = &url.path()[1..];

                    let mut archive = zip.lock().unwrap();
                    let file_result = archive.by_name(path);
                    if let Ok(mut file) = file_result {
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
