use crate::utils;
use crate::Profile;
use rand::random;

pub fn coinflip(profile: Profile) -> Profile {
    let choice = match utils::option_menu("What side do you chose", vec!["Heads", "Tails"]) {
        "Heads" => true,
        "Tails" => false,
        _ => panic!("how tf did we get here"),
    };
    if choice == random::<bool>() {
        println!("win");
    } else {
        println!("lose");
    }
    profile
}
