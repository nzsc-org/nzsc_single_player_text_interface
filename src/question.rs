use nzsc_single_player::io::Question;

use std::fmt::Write;

pub fn to_string(question: &Question) -> String {
    let mut s = String::new();

    match question {
        &Question::ChooseCharacter {
            ref available_characters,
        } => {
            write!(s, "Choose a character:").unwrap();
            for character in available_characters {
                write!(s, "\n\t{}", character).unwrap();
            }
        },
        &Question::ChooseBooster {
            ref available_boosters,
        } => {
            write!(s, "Choose a booster:").unwrap();
            for booster in available_boosters {
                write!(s, "\n\t{}", booster).unwrap();
            }
        },
        &Question::ChooseMove {
            ref available_moves,
        } => {
            write!(s, "Choose a move:").unwrap();
            for available_move in available_moves {
                write!(s, "\n\t{}", available_move).unwrap();
            }
        },
    }

    s
}
