use serde::Serialize;
use std::borrow::Cow;

use crate::Visualization;

pub enum True {
    True,
}

impl Default for True {
    fn default() -> Self {
        True::True
    }
}

impl Serialize for True {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(true)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Text<'t> {
    kind: TextKind,
    text: Cow<'t, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name: Option<String>,
}

#[derive(Serialize, Default)]
pub struct TextKind {
    text: True,
}

impl<'t> Text<'t> {
    pub fn new(text: Cow<'t, str>) -> Self {
        Self {
            text,
            file_name: None,
            kind: TextKind::default(),
        }
    }

    pub fn with_file_name(&mut self, file_name: String) -> &mut Text<'t> {
        self.file_name = Some(file_name);
        self
    }
}

impl<'t> Visualization for Text<'t> {
    fn json_data(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PngImage<'t> {
    kind: PngImageKind,
    base64_data: Cow<'t, str>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PngImageKind {
    image_png: True,
}

impl<'t> PngImage<'t> {
    pub fn new(png_data: &[u8]) -> Self {
        Self {
            kind: PngImageKind::default(),
            base64_data: base64::encode(png_data).into(),
        }
    }
}

impl<'t> Visualization for PngImage<'t> {
    fn json_data(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
