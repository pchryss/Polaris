use ratatui::{
    layout::Rect, style::{Color, Style}, text::Line, widgets::{Block, BorderType, Borders, Paragraph}, Frame
};
pub mod constellations;

use crate::constellations::Constellation;

pub fn draw_constellation(frame: &mut Frame, constellation: &Constellation) {
    //let text = constellation.pattern.join("\n");
    let size = frame.area();
    let x = size.width.saturating_sub(50) / 2;
    let y = size.height.saturating_sub(20) / 2;
    let area = Rect {
        x,
        y,
        width: 50,
        height: 20,
    };


    let block = Block::new()
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .title(Line::from("Polaris").centered());
    frame.render_widget(block, area);
}
