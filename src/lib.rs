use color_eyre::owo_colors::colors::xterm;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, BorderType, Borders, Paragraph}, Frame
};
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
