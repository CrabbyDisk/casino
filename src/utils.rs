use std::fs::{self, File};
use std::io::{stdin, stdout};
use std::path::Path;

use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor};
use crossterm::terminal::{enable_raw_mode, Clear, ClearType};

use crate::{CasinoError, Profile};

pub fn load_profile(path: &str) -> Result<Profile, CasinoError> {
    if !Path::new(path).exists() {
        return Err(CasinoError::NoSaveFile);
    }
    let data = fs::read_to_string(path).unwrap();
    let keys: Vec<&str> = data.lines().collect();
    Ok(Profile {
        money: keys[0].parse().unwrap(),
        xp: keys[1].parse().unwrap(),
        name: keys[2].to_string(),
    })
}

pub fn save_profile(profile: &Profile, path: &str) {
    let data = format!("{}\n{}\n{}", profile.money, profile.xp, profile.name);
    fs::write(path, data);
}

pub fn option_menu<'a>(prompt: &str, options: Vec<&'a str>) -> &'a str {
    let mut selected: usize = 0;
    enable_raw_mode().unwrap();
    execute!(stdout(), MoveTo(0, 0), Clear(ClearType::Purge));
    loop {
        // Display Code
        execute!(stdout(), Print(format!("{}\n", prompt)), MoveToNextLine(1));
        for (i, option) in options.iter().enumerate() {
            if i == selected {
                execute!(stdout(), SetBackgroundColor(Color::Red),);
            }
            execute!(stdout(), Print(option), ResetColor, MoveToNextLine(0));
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < options.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    return options[selected];
                }
                _ => {}
            }
        }

        // Clear the terminal after each input
        execute!(stdout(), Clear(ClearType::Purge), MoveTo(0, 0)).unwrap();
    }
}

pub fn ask_amount(profile: &Profile) -> u32 {
    println!("How much money do you want to bet");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    if input.parse::<u32>().unwrap() >= profile.money {
        panic!();
    } else {
        return input.parse::<u32>().unwrap();
    }
}
