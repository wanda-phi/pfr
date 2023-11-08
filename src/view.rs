use winit::event::{ElementState, TouchPhase};
use winit::keyboard::KeyCode;

use crate::config::{HighScore, Options, TableId};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Route {
    Intro(Option<TableId>),
    Table(TableId),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Action {
    None,
    Navigate(Route),
    Exit,
    SaveOptions(Options),
    SaveHighScores(TableId, [HighScore; 4]),
}

pub trait View {
    fn get_resolution(&self) -> (u32, u32);
    fn get_fps(&self) -> u32;
    fn run_frame(&mut self) -> Action;
    fn handle_touch(&mut self, id: u64, phase: TouchPhase, pos: (i32, i32));
    fn handle_key(&mut self, key: KeyCode, state: ElementState);
    fn render(&self, data: &mut [u8], pal: &mut [(u8, u8, u8)]);
}
