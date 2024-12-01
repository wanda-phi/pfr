use crate::config::ConfigStore;
use js_sys::{Array, Uint8Array, JSON};
use std::path::PathBuf;
use wasm_bindgen::prelude::*;

pub struct WasmConfigStore {
    pub path: PathBuf,
    local_storage: web_sys::Storage,
}

impl WasmConfigStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let window = web_sys::window();
        let local_storage = window.unwrap().local_storage().unwrap().unwrap();
        Self {
            path: path.into(),
            local_storage,
        }
    }
}

impl ConfigStore for WasmConfigStore {
    fn load(&self, name: &str) -> Option<Vec<u8>> {
        let value = self.local_storage.get_item(name);
        let buffer: JsValue = JSON::parse(&value.unwrap()?).ok()?;

        let array = Uint8Array::new(&buffer);
        Some(array.to_vec())
    }
    fn save(&self, name: &str, data: &[u8]) {
        let array = Array::from(&Uint8Array::from(data));

        let buffer: String = JSON::stringify(&JsValue::from(&array)).unwrap().into();
        let _ = self.local_storage.set_item(&name, &buffer);
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn get_asset(s: &str) -> Vec<u8>;
}

#[wasm_bindgen]
extern "C" {
    pub fn bind_mobile_events();
}
