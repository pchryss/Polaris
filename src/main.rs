use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use polaris::load_data;
use polaris::menu::*;
use polaris::game::*;
use polaris::planetarium::*;
use polaris::constellations::*;
use polaris::save_data;
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
                KeyCode::Char('e') | KeyCode::Char('E') => {
                    play(&mut terminal)?
                }
                KeyCode::Char('p') | KeyCode::Char('P') => {
                    planetarium(&mut terminal)?
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

fn planetarium(terminal: &mut DefaultTerminal) -> Result<()>{

    let mut selected = 0;

    loop {
        terminal.draw(|f| draw_planeterium(f, selected))?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        break Ok(());
                    }
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected < CONSTELLATIONS.len() - 1 {
                            selected += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn play(terminal: &mut DefaultTerminal) -> Result<()>{
    let mut input = String::new();
    let mut constellation = &CONSTELLATIONS[rand::rng().random_range(0..CONSTELLATIONS.len())];
    let mut result = GuessResult::NoGuess;
    let mut result_changed_at: Option<Instant> = None;
    let mut data = load_data();
    let mut answer = "";

    loop {
        terminal.draw(|f| draw_game(f, constellation, &input, &result, answer))?;
        
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
                            true => {
                                data.insert(constellation.name.to_string());
                                GuessResult::Correct
                            }
                            false => {
                                answer = constellation.name;
                                GuessResult::Incorrect
                            }
                        };
                        input.clear();
                        result_changed_at = Some(Instant::now());
                        constellation = &CONSTELLATIONS[rand::rng().random_range(0..CONSTELLATIONS.len())];
                    }
                    KeyCode::Esc => {
                        save_data(&data);
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
                answer = "";
            }
        }
    }
}
