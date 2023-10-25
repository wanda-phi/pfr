pub mod assets;
pub mod bcd;
pub mod config;
pub mod icons;
pub mod intro;
pub mod sound;
pub mod table;
pub mod view;

#[cfg(target_arch = "wasm32")]
pub mod wasm;
