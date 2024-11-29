use crate::utils;
use crate::Profile;
use rand::random;
use strum_macros::{EnumMessage, VariantArray};

#[derive(EnumMessage, VariantArray, Clone)]
enum CoinFlipOption {
    #[strum(message = "Heads")]
    Heads,
    #[strum(message = "Tails")]
    Tails,
}

pub fn coinflip(mut profile: Profile) -> Profile {
    let amount = utils::ask_amount(&profile);
    let choice = match utils::option_menu::<CoinFlipOption>("What side do you chose") {
        CoinFlipOption::Heads => true,
        CoinFlipOption::Tails => false,
    };
    if choice == random::<bool>() {
        println!("win");
        profile.money += amount;
    } else {
        println!("lose");
        profile.money -= amount;
    }
    profile
}
