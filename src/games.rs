use crate::utils;
use crate::Profile;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::VariantArray;
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
    match utils::option_menu::<CoinFlipOption>("What side do you chose") {
        CoinFlipOption::Heads => {
            println!("win");
            profile.money += amount;
        }
        CoinFlipOption::Tails => {
            println!("lose");
            profile.money -= amount;
        }
    };
    profile
}

#[derive(VariantArray, Clone, PartialEq, Debug)]
enum Slots {
    Diamond,
    Cherry,
    Lemon,
    Bell,
    Lose,
}

pub fn slot(mut profile: Profile) -> Profile {
    let amount = utils::ask_amount(&profile);
    let mut rng = thread_rng();
    let mut slots_iter = Slots::VARIANTS.choose_multiple(&mut rng, 3).cloned();

    let (slot_one, slot_two, slot_three) = (
        slots_iter.next().unwrap(),
        slots_iter.next().unwrap(),
        slots_iter.next().unwrap(),
    );
    let mut multiplier: f32 = 0.0;
    multiplier += match slot_one {
        Slots::Diamond => 3.0,
        Slots::Cherry => 1.0,
        Slots::Lemon => 1.25,
        Slots::Bell => 1.5,
        Slots::Lose => 0.0,
    };
    multiplier += match slot_two {
        Slots::Diamond => 3.0,
        Slots::Cherry => 1.0,
        Slots::Lemon => 1.25,
        Slots::Bell => 1.5,
        Slots::Lose => 0.0,
    };
    multiplier += match slot_three {
        Slots::Diamond => 3.0,
        Slots::Cherry => 1.0,
        Slots::Lemon => 1.25,
        Slots::Bell => 1.5,
        Slots::Lose => 0.0,
    };

    println!("{:?} {:?} {:?}", slot_one, slot_two, slot_three);
    if slot_one == Slots::Lose || slot_two == Slots::Lose || slot_three == Slots::Lose {
        profile.money -= amount;
        println!("You lost {amount}");
    } else {
        profile.money += (amount as f32 * multiplier) as u32;
        println!("You won {}", (amount as f32 * multiplier) as u32);
    }

    profile
}
