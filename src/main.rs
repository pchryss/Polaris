use polaris::{constellations::{CONSTELLATIONS}, draw_constellation};
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal,
};
use rand::{self, Rng};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        let constellation = &CONSTELLATIONS[rand::rng().random_range(0..CONSTELLATIONS.len())];
        terminal.draw(|f| draw_constellation(f, constellation))?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}
