use ratatui::{
    layout::{Alignment,Rect}, style::{Color, Modifier, Style}, text::{Span}, widgets::{Block,Borders, List, ListItem, ListState, Paragraph}, Frame
};

use crate::{constellations::{self, CONSTELLATIONS, UNKNOWN}, draw_border, game::draw_constellation, load_data};
use crate::draw_exit_instructions;
use crate::constellations::Constellation;

pub fn draw_planeterium(frame: &mut Frame, selected: usize) {
    let size = frame.area();
    let width = 60;
    let height = 25;
    let x = size.width.saturating_sub(width) / 2;
    let y = size.height.saturating_sub(height) / 2;
    let area = Rect {
        x,
        y,
        width,
        height,
    };
    if size.width < 60 || size.height < 25 {
        let warning = Paragraph::new("Terminal too small!")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(warning, size);
        return;
    }
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
        width: inner_area.width / 3,
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
    let data = load_data();
    let block = Block::new().borders(Borders::ALL);
    frame.render_widget(block, area);
    let constellation = if data.contains(CONSTELLATIONS[selected].name) {
        &CONSTELLATIONS[selected]
    } else {
        &UNKNOWN
    };
    draw_constellation(frame, &constellation, area);

}

pub fn draw_list(frame: &mut Frame, area: Rect, selected: usize) {
    
    let data = load_data();

    let block = Block::new().borders(Borders::ALL);
    frame.render_widget(block, area);

    let mut items: Vec<ListItem> = Vec::new();
    for i in 0..CONSTELLATIONS.len() {
        let style = if i == selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let s = if data.contains(CONSTELLATIONS[i].name) {
            CONSTELLATIONS[i].name.to_string()
        } else {
            "?????????".to_string()
        };
        items.push(ListItem::new(Span::styled(s, style)));
    }

    let list = List::new(items);
    
    let mut state = ListState::default();
    state.select(Some(selected));
    let list_area = Rect {
        x: area.x + 2,
        y: area.y + 1,
        width: area.width - 2,
        height: area.height - 2
    };
    frame.render_stateful_widget(list, list_area, &mut state);
}