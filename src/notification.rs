use nzsc_single_player::io::{
    Notification,
    WhoGetsTheHeadstart,
    WhoGetsThePoint,
};

pub fn to_string(notification: &Notification) -> String {
    match notification {
        &Notification::CharacterSelectionAndHeadstart {
            ref human_character,
            ref computer_character,
            ref who_gets_the_headstart,
        } => {
            let who_gets_the_headstart = match who_gets_the_headstart {
                WhoGetsTheHeadstart::Neither => "neither of you get the headstart",
                WhoGetsTheHeadstart::JustComputer => "the computer gets the headstart",
                WhoGetsTheHeadstart::JustHuman => "you get the headstart",
            };
            format!("You chose {}.\nThe computer chose {}.\nAs a result, {}.", human_character, computer_character, who_gets_the_headstart)
        },

        &Notification::SameCharacterSelection {
            ref both_character,
        } => {
            format!("Both of you chose {}.\nAs a result, both of you must repick.", both_character)
        },

        &Notification::BoosterSelection {
            ref human_booster,
            ref computer_booster,
        } => {
            format!("You chose {}.\nThe computer chose {}.\nLet the battle begin!", human_booster, computer_booster)
        },

        &Notification::MoveSelectionAndOutcome {
            ref human_move,
            ref computer_move,
            ref who_gets_the_point,
        } => {
            let who_gets_the_point = match who_gets_the_point {
                WhoGetsThePoint::Neither => "neither of you get a point",
                WhoGetsThePoint::JustComputer => "the computer gets a point",
                WhoGetsThePoint::JustHuman => "you get a point",
                WhoGetsThePoint::Both => "both of you get a point"
            };

            format!("You chose {}.\nThe computer chose {}.\nAs a result, {}.", human_move, computer_move, who_gets_the_point)
        },

        &Notification::ScoreUpdate {
            ref human_points,
            ref computer_points,
        } => {
            format!("The score is now {}-{}.", human_points, computer_points)
        },

        &Notification::TiebreakingScoreSetback {
            ref both_points,
        } => {
            format!("Both of you are tied at {0}-{0}.\nAs a result, the score has been set back to 4-4.", both_points)
        },

        &Notification::GameOver {
            ref human_points,
            ref computer_points,
        } => {
            fn nickname_for_margin(margin: u8) -> String {
                match margin {
                    1 => "Clinch".to_string(),
                    2 => "Hypnotization".to_string(),
                    3 => "Obliteration".to_string(),
                    4 => "Annihilation".to_string(),
                    5 => "Wipeout".to_string(),
                    _ => {
                        panic!("Impossible victory margin: {}", margin);
                    }
                }
            }

            if human_points > computer_points {
                let margin_nickname = nickname_for_margin(human_points - computer_points);
                format!("You won {}-{} ({}).", human_points, computer_points, margin_nickname)
            } else {
                let margin_nickname = nickname_for_margin(computer_points - human_points);
                format!("You lost {}-{} ({}).", human_points, computer_points, margin_nickname)
            }
        },

        &Notification::CharacterNonexistentPenalty {
            ref attempted_character_name,
        } => {
            format!("\"{}\" is not a character.\n4 wait penalty!", attempted_character_name)
        },

        &Notification::CharacterThreeTimesInARowPenalty {
            ref attempted_character,
        } => {
            format!("You already chose {} 3 times in a row.\nYou must choose another character before choosing it again.\n3 wait penalty!", attempted_character)
        },

        &Notification::BoosterNonexistentPenalty {
            ref attempted_booster_name,
        } => {
            format!("\"{}\" is not a booster.\n4 wait penalty!", attempted_booster_name)
        },

        &Notification::BoosterFromWrongCharacterPenalty {
            ref attempted_booster,
        } => {
            format!("{} is from a character you did not choose.\n3 wait penalty!", attempted_booster)
        },

        &Notification::MoveNonexistentPenalty {
            ref attempted_move_name,
        } => {
            format!("\"{}\" is not a move.\n4 wait penalty!", attempted_move_name)
        },

        &Notification::MoveThreeTimesInARowPenalty {
            ref attempted_move,
        } => {
            format!("You already chose {} 3 times in a row. You must choose another move before choosing it again.\n3 wait penalty!", attempted_move)
        },

        &Notification::MoveSingleUsePenalty {
            ref attempted_move,
        } => {
            format!("{} is single-use. You cannot use it again.\n4 wait penalty!", attempted_move)
        },

        &Notification::MoveDestroyedPenalty {
            ref attempted_move,
        } => {
            format!("{} has been destroyed. You cannot use it.\n4 wait penalty!", attempted_move)
        },

        &Notification::MoveFromWrongCharacterPenalty {
            ref attempted_move,
        } => {
            format!("{} is from a character that you did not choose.\n3 wait penalty!", attempted_move)
        },

        &Notification::MoveFromWrongBoosterPenalty {
            ref attempted_move,
        } => {
            format!("{} is from a booster that you did not choose.\n2 wait penalty!", attempted_move)
        },
    }
}
