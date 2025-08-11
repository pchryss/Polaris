use color_eyre::owo_colors::colors::xterm;
use ratatui::{
    layout::{Alignment, Rect}, style::{Color, Style}, text::Line, widgets::{Block, BorderType, Borders, Paragraph}, Frame
};
pub mod constellations;

use crate::constellations::Constellation;

pub fn draw_polaris(frame: &mut Frame, constellation: &Constellation) {
    let size = frame.area();
    let width = 50;
    let height = 20;
    let x = size.width.saturating_sub(width) / 2;
    let y = size.height.saturating_sub(height) / 2;
    let area = Rect {
        x,
        y,
        width,
        height,
    };
    draw_border(frame, area);
    draw_constellation(frame, constellation, area);
}

pub fn draw_border(frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .title(Line::from("Polaris").centered());
    frame.render_widget(block, area);
}

pub fn draw_constellation(frame: &mut Frame, constellation: &Constellation, area: Rect) {
    let lines = constellation.pattern.len() as u16;
    let vertical_padding = (area.height - lines) / 2;
    let mut padded_text = String::new();
    for _ in 0..vertical_padding {
        padded_text.push('\n');
    }
    padded_text.push_str(&constellation.pattern.join("\n"));
    let paragraph = Paragraph::new(padded_text).alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}
