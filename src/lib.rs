use color_eyre::owo_colors::colors::xterm;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, BorderType, Borders, Paragraph}, Frame
};
pub mod constellations;
pub mod menu;
pub mod game;


pub fn draw_border(frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    frame.render_widget(block, area);
}
