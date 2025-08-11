use polaris::{constellations::{Constellation, CONSTELLATIONS}, draw_constellation};
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|f| draw_constellation(f, &CONSTELLATIONS[0]))?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}
