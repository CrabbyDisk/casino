use std::fs::File;
use std::fs;
use std::path::Path;
use std::io::{self, Read};

const SAVE_FILE: &str = "savefile.txt";

enum CasinoError {
    NoSaveFile,
    BadSaveFile,
}

#[derive(Debug)]
struct Profile {
    money: u32,
    xp: u32,
    name: String,
}

fn main() {
    println!("Hello, world!");
    let mut profile: Profile = match load_profile() {
        Ok(x) => x,
        Err(CasinoError::NoSaveFile) => new_profile(),
        Err(CasinoError::BadSaveFile) => new_profile(),
    };
    dbg!(profile);

}

fn load_profile() -> Result<Profile, CasinoError> {
    if !Path::new(SAVE_FILE).exists() {
       return Err(CasinoError::NoSaveFile);
    }
    let data = fs::read_to_string(SAVE_FILE).unwrap();
    let keys: Vec<&str> = data.lines().collect();
    Ok(Profile {money: keys[0].parse().unwrap(), xp: keys[1].parse().unwrap(), name: keys[2].to_string()})
}

fn save_profile(profile: Profile) {
    let data = format!("{}\n{}\n{}", profile.money, profile.xp, profile.name);
    fs::write(SAVE_FILE, data);
}

fn new_profile() -> Profile {
    Profile {money: 0, xp: 0, name: "bob".to_string()}
}
