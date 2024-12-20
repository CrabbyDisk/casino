use strum_macros::{EnumMessage, VariantArray};

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

#[derive(VariantArray, EnumMessage, Clone)]
enum MainMenuOptions {
    #[strum(message = "Coin Flip")]
    CoinFlip,

    #[strum(message = "Slot Machine")]
    SlotMachine,

    #[strum(message = "Quit")]
    Quit,
}

fn main() {
    let mut profile: Profile = match utils::load_profile(SAVE_FILE) {
        Ok(x) => x,
        Err(CasinoError::NoSaveFile) => new_profile(),
        Err(CasinoError::BadSaveFile) => new_profile(),
    };

    loop {
        profile = match utils::option_menu::<MainMenuOptions>("What do you want to do") {
            MainMenuOptions::CoinFlip => games::coinflip(profile),
            MainMenuOptions::SlotMachine => games::slot(profile),
            MainMenuOptions::Quit => break,
        };
    }

    utils::save_profile(&profile, SAVE_FILE);
}

fn new_profile() -> Profile {
    Profile {
        money: 0,
        xp: 0,
        name: "bob".to_string(),
    }
}
