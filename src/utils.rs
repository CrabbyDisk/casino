use std::fs::{self};
use std::io::{stdin, stdout};
use std::path::Path;

use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use strum::{EnumMessage, VariantArray};

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
    fs::write(path, data).unwrap();
}

pub fn ask_amount(profile: &Profile) -> u32 {
    println!("How much money do you want to bet");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    if input.parse::<u32>().unwrap() >= profile.money {
        panic!("broke lol");
    }
    input.parse::<u32>().unwrap()
}

pub fn option_menu<T: VariantArray + EnumMessage + Clone>(prompt: &str) -> T {
    let mut selected: usize = 0;
    enable_raw_mode().unwrap();
    execute!(stdout(), MoveTo(0, 0), Clear(ClearType::Purge)).unwrap();
    loop {
        execute!(stdout(), Print(format!("{}\n", prompt)), MoveToNextLine(1)).unwrap();
        for (i, option) in T::VARIANTS.iter().enumerate() {
            if i == selected {
                execute!(stdout(), SetBackgroundColor(Color::Red),).unwrap();
            }
            execute!(
                stdout(),
                Print(
                    option
                        .get_message()
                        .expect("No String was assigned to option")
                ),
                ResetColor,
                MoveToNextLine(0)
            )
            .unwrap();
        }
        if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Up => {
                    selected = selected.saturating_sub(1);
                }
                KeyCode::Down => {
                    if selected < T::VARIANTS.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    break;
                }
                _ => {}
            }
        }

        // Clear the terminal after each input
        execute!(stdout(), Clear(ClearType::Purge), MoveTo(0, 0)).unwrap();
    }
    disable_raw_mode().unwrap();
    T::VARIANTS[selected].clone()
}
