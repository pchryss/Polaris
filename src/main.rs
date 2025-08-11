use polaris::{constellations::CONSTELLATIONS, draw_constellation, draw_polaris};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
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

    let mut input = String::new();
    let constellation = &CONSTELLATIONS[rand::rng().random_range(0..CONSTELLATIONS.len())];

    loop {
        terminal.draw(|f| draw_polaris(f, constellation, &input))?;
            if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    input.push(c);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Enter => {
                    match input.eq_ignore_ascii_case(constellation.name) {
                        true => println!("Correct!"),
                        false => println!("Incorrect :(")
                    }
                }
                KeyCode::Esc => {
                    break Ok(());
                }
                _ => {}
            }
        }
    }
}
