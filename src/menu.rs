use color_eyre::owo_colors::colors::xterm;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, BorderType, Borders, Paragraph}, Frame
};
use crate::draw_border;
pub fn draw_menu(frame: &mut Frame) {
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
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width - 2,
        height: area.height - 2
    };
    let instructions_height = 4;
    let welcome_area = Rect {
        x: inner_area.x,
        y: inner_area.y,
        width: inner_area.width,
        height: inner_area.height - instructions_height,
    };

    let instructions_area = Rect {
        x: inner_area.x,
        y: inner_area.y + welcome_area.height,
        width: inner_area.width,
        height: instructions_height
    };
    draw_border(frame, area);
    draw_welcome(frame, welcome_area);
    draw_welcome_instructions(frame, instructions_area);

}

pub fn draw_welcome_instructions(frame: &mut Frame, area: Rect) {

    let paragraph = Paragraph::new("E -> Explore the Stars\n  P -> Planetarium\n ESC -> Exit Polaris")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}

pub fn draw_welcome(frame: &mut Frame, area: Rect) {
    let welcome_ascii = r#"
  _    _      _                            _        
 | |  | |    | |                          | |       
 | |  | | ___| | ___ ___  _ __ ___   ___  | |_ ___  
 | |/\| |/ _ \ |/ __/ _ \| '_ ` _ \ / _ \ | __/ _ \ 
 \  /\  /  __/ | (_| (_) | | | | | |  __/ | || (_) |
  \/  \/ \___|_|\___\___/|_| |_| |_|\___|  \__\___/ 
    ______ _____ _       ___  ______ _____ _____    
    | ___ \  _  | |     / _ \ | ___ \_   _/  ___|   
    | |_/ / | | | |    / /_\ \| |_/ / | | \ `--.    
    |  __/| | | | |    |  _  ||    /  | |  `--. \   
    | |   \ \_/ / |____| | | || |\ \ _| |_/\__/ /   
    \_|    \___/\_____/\_| |_/\_| \_|\___/\____/    "#;
        let welcome = Paragraph::new(welcome_ascii)
            .alignment(Alignment::Center);
        frame.render_widget(welcome, area);
}