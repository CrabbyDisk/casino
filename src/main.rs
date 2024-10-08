mod games;
mod utils;

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
    let mut profile: Profile = match utils::load_profile(SAVE_FILE) {
        Ok(x) => x,
        Err(CasinoError::NoSaveFile) => new_profile(),
        Err(CasinoError::BadSaveFile) => new_profile(),
    };

    utils::save_profile(&profile, SAVE_FILE);

    match utils::option_menu("What do you want do play?", vec!["Coin flip", "Quit"]) {
        "Coin flip" => games::coinflip(profile),
        "Quit" => return,
        _ => panic!("wtf just happened here, HOW TF DID WE GET HERE"),
    };
}

fn menu(profile: Profile) {}

fn new_profile() -> Profile {
    Profile {
        money: 0,
        xp: 0,
        name: "bob".to_string(),
    }
}
