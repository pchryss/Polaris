use color_eyre::owo_colors::colors::xterm;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, BorderType, Borders, Paragraph}, Frame
};

use crate::{constellations::CONSTELLATIONS, draw_border, game::draw_constellation};
use crate::draw_exit_instructions;
use crate::constellations::Constellation;

pub fn draw_planeterium(frame: &mut Frame, selected: usize) {
    let size = frame.area();
    let width = 60;
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
    let instructions_height = 1;
    let instructions_area = Rect {
        x: inner_area.x,
        y: inner_area.y,
        width: inner_area.width,
        height: instructions_height,
    };
    let list_area = Rect {
        x: inner_area.x,
        y: inner_area.y + instructions_area.height,
        width: inner_area.width / 4,
        height: inner_area.height - instructions_area.height
    };
    let constellation_area = Rect {
        x: inner_area.x + list_area.width,
        y: inner_area.y + instructions_area.height,
        width: inner_area.width - list_area.width,
        height: inner_area.height - instructions_area.height
    };
    draw_exit_instructions(frame, instructions_area);
    draw_list(frame, list_area, selected);
    draw_selected(frame, constellation_area, selected);
}

pub fn draw_selected(frame: &mut Frame, area: Rect, selected: usize) {
    let block = Block::new().borders(Borders::ALL);
    frame.render_widget(block, area);
    draw_constellation(frame, &CONSTELLATIONS[selected], area);

}

pub fn draw_list(frame: &mut Frame, area: Rect, selected: usize) {
    let block = Block::new().borders(Borders::ALL);
    let mut list: Vec<Line> = Vec::new();
    for i in 0..CONSTELLATIONS.len() {
        let span = if i == selected {
            Line::styled(
                CONSTELLATIONS[i].name.to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            )
        } else {
            Line::raw(CONSTELLATIONS[i].name)
        };
        list.push(span);

    }
    frame.render_widget(block, area);
    let list = Paragraph::new(list)
        .alignment(Alignment::Center)
        .block(Block::new().borders(Borders::ALL));
    frame.render_widget(list, area);
}