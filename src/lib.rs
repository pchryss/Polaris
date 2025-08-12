use std::{collections::HashSet, fs, path::PathBuf};

use color_eyre::owo_colors::colors::xterm;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, BorderType, Borders, Paragraph}, Frame
};

use crate::constellations::Constellation;
pub mod constellations;
pub mod menu;
pub mod game;
pub mod planetarium;


pub fn draw_border(frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    frame.render_widget(block, area);
}

pub fn draw_exit_instructions(frame: &mut Frame, area: Rect) {
    let instructions = Span::styled("< Esc", Style::default().fg(Color::Yellow));
    let paragraph = Paragraph::new(instructions);
    frame.render_widget(paragraph, area);
}

fn storage_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Cannt find home directory");
    path.push(".polaris.json");
    path
}

pub fn load_data() -> HashSet<String> {
    let path = storage_path();
    if let Ok(data) = fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        HashSet::new()
    }
}

pub fn save_data(data: &HashSet<String>) {
    let path = storage_path();
    let data = serde_json::to_string_pretty(data).unwrap();
    fs::write(path, data).unwrap();
}
