use std::time::{Duration, Instant};

use polaris::{constellations::CONSTELLATIONS, draw_constellation, draw_menu, draw_polaris, GuessResult};
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
    loop {
        terminal.draw(|f| draw_menu(f))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(' ') => {
                    play(&mut terminal)?
                }
                KeyCode::Esc => {
                    return Ok(())
                }
                _ => {

                }
            }
        }
    }
    //play(terminal)
}

fn play(terminal: &mut DefaultTerminal) -> Result<()>{
    let mut input = String::new();
    let mut constellation = &CONSTELLATIONS[rand::rng().random_range(0..CONSTELLATIONS.len())];
    let mut result = GuessResult::NoGuess;
    let mut result_changed_at: Option<Instant> = None;
    loop {
        terminal.draw(|f| draw_polaris(f, constellation, &input, &result))?;
        
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        result = match input.eq_ignore_ascii_case(constellation.name) {
                            true => GuessResult::Correct,
                            false => GuessResult::Incorrect
                        };
                        input.clear();
                        result_changed_at = Some(Instant::now());
                        constellation = &CONSTELLATIONS[rand::rng().random_range(0..CONSTELLATIONS.len())];
                    }
                    KeyCode::Esc => {
                        break Ok(());
                    }
                    _ => {}
                }
            }
        }

        if let Some(time) = result_changed_at {
            if time.elapsed() >= Duration::from_millis(1000) {
                result = GuessResult::NoGuess;
                result_changed_at = None;
            }
        }
    }
}
