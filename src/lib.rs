use color_eyre::owo_colors::colors::xterm;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::Line, widgets::{Block, BorderType, Borders, Paragraph}, Frame
};
pub mod constellations;

use crate::constellations::Constellation;

pub fn draw_polaris(frame: &mut Frame, constellation: &Constellation, input: &str) {
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
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width - 2,
        height: area.height - 2
    };
    let input_height = 3;
    let constellation_area = Rect {
        x: inner_area.x,
        y: inner_area.y,
        width: inner_area.width,
        height: inner_area.height - input_height,
    };
    let input_area = Rect {
        x: inner_area.x,
        y: inner_area.y + constellation_area.height,
        width: inner_area.width,
        height: input_height,
    };

    draw_constellation(frame, constellation, constellation_area);
    draw_input(frame, input, input_area);
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

pub fn draw_input(frame: &mut Frame, input: &str, area: Rect) {
    let input_box = Paragraph::new(input)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(input_box, area);
}
