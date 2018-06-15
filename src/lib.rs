extern crate nzsc2p;
use nzsc2p::two_player_game::{
    NZSCTwoPlayerGame,
    Phase,
};
use nzsc2p::players::{
    CharacterlessPlayer,
    BoosterlessPlayer,
    MovelessPlayer,
};
use nzsc2p::streaks::{ CharacterStreak, MoveStreak, };

use std::fmt::Display;

mod tests;

fn opt_to_json_null_or_string<T: Display>(opt: Option<T>) -> String {
    if let Some(some) = opt {
        format!("\"{}\"", some)
    } else {
        "null".to_string()
    }
}

fn character_streak_to_json_obj(streak: &CharacterStreak) -> String {
    format!(
        r#"{{"times":{},"character":{}}}"#,
        streak.times,
        opt_to_json_null_or_string(streak.repeated_character)
    )
}

fn move_streak_to_json_obj(streak: &MoveStreak) -> String {
    format!(
        r#"{{"times":{},"move":{}}}"#,
        streak.times,
        opt_to_json_null_or_string(streak.repeated_move)
    )
}

fn vec_to_json_array<T: Display>(vec: Vec<T>) -> String {
    let with_quotes: Vec<String> = vec.into_iter().map(|x| format!("\"{}\"", x)).collect();
    let mut array = "[".to_string();
    for item in &with_quotes {
        array.push_str(item);
        array.push_str(",")
    }
    if with_quotes.len() > 0 {
        array.pop();
    }
    array.push_str("]");
    array
}

fn characterless_player_to_phase_json_obj(player: &CharacterlessPlayer) -> String {
    format!(
        r#"{{"points":{},"waits":{},"character_streak":{},"selected_character":{}}}"#,
        player.points,
        player.waits,
        character_streak_to_json_obj(&player.character_streak),
        opt_to_json_null_or_string(player.selected_character)
    )
}

fn boosterless_player_to_phase_json_obj(player: &BoosterlessPlayer) -> String {
    format!(
        r#"{{"points":{},"waits":{},"character":"{}","selected_booster":{}}}"#,
        player.points,
        player.waits,
        player.character,
        opt_to_json_null_or_string(player.selected_booster)
    )
}

fn moveless_player_to_phase_json_obj(player: &MovelessPlayer) -> String {
    format!(
        r#"{{"points":{},"waits":{},"character":"{}","booster":"{}","move_streak":{},"destroyed_moves":{},"selected_move":{}}}"#,
        player.points,
        player.waits,
        player.character,
        player.booster,
        move_streak_to_json_obj(&player.move_streak),
        vec_to_json_array(player.destroyed_moves.clone()),
        opt_to_json_null_or_string(player.selected_move)
    )
}

fn characterless_player_to_question_json_obj(player: &CharacterlessPlayer) -> String {
    if let Some(ref _selected_character) = &player.selected_character {
        r#"{"question":null}"#.to_string()
    } else {
        format!(r#"{{"question":"CHOOSE_CHARACTER","available_characters":{}}}"#, vec_to_json_array(player.available_characters()))
    }
}

fn boosterless_player_to_question_json_obj(player: &BoosterlessPlayer) -> String {
    if let Some(ref _selected_booster) = &player.selected_booster {
        r#"{"question":null}"#.to_string()
    } else {
        format!(r#"{{"question":"CHOOSE_BOOSTER","available_boosters":{}}}"#, vec_to_json_array(player.available_boosters()))
    }
}

fn moveless_player_to_question_json_obj(player: &MovelessPlayer) -> String {
    if let Some(ref _selected_move) = &player.selected_move {
        r#"{"question":null}"#.to_string()
    } else {
        format!(r#"{{"question":"CHOOSE_MOVE","available_moves":{}}}"#, vec_to_json_array(player.available_moves()))
    }
}

pub fn phase_as_json(game: &NZSCTwoPlayerGame) -> String {
    match &game.phase {
        &Phase::CharacterChoosing(ref a, ref b) => {
            format!(
                r#"{{"phase":"CHARACTER_CHOOSING","a":{},"b":{}}}"#,
                characterless_player_to_phase_json_obj(a),
                characterless_player_to_phase_json_obj(b)
            )
        },

        &Phase::BoosterChoosing(ref a, ref b) => {
            format!(
                r#"{{"phase":"BOOSTER_CHOOSING","a":{},"b":{}}}"#,
                boosterless_player_to_phase_json_obj(a),
                boosterless_player_to_phase_json_obj(b)
            )
        },

        &Phase::MoveChoosing(ref a, ref b) => {
            format!(
                r#"{{"phase":"MOVE_CHOOSING","a":{},"b":{}}}"#,
                moveless_player_to_phase_json_obj(a),
                moveless_player_to_phase_json_obj(b)
            )
        },

        &Phase::GameOver(ref a, ref b) => {
            format!(
                r#"{{"phase":"GAME_OVER","a":{},"b":{}}}"#,
                a,
                b
            )
        },
    }
}

pub fn question_as_json(game: &NZSCTwoPlayerGame) -> String {
    match &game.phase {
        &Phase::CharacterChoosing(ref a, ref b) => {
            format!(
                r#"{{"phase":"CHARACTER_CHOOSING","a":{},"b":{}}}"#,
                characterless_player_to_question_json_obj(a),
                characterless_player_to_question_json_obj(b)
            )
        },

        &Phase::BoosterChoosing(ref a, ref b) => {
            format!(
                r#"{{"phase":"BOOSTER_CHOOSING","a":{},"b":{}}}"#,
                boosterless_player_to_question_json_obj(a),
                boosterless_player_to_question_json_obj(b)
            )
        },

        &Phase::MoveChoosing(ref a, ref b) => {
            format!(
                r#"{{"phase":"MOVE_CHOOSING","a":{},"b":{}}}"#,
                moveless_player_to_question_json_obj(a),
                moveless_player_to_question_json_obj(b)
            )
        },

        &Phase::GameOver(ref _a, ref _b) => {
            format!(
                r#"{{"phase":"GAME_OVER","a":{0},"b":{0}}}"#,
                r#"{"question":null}"#
            )
        },
    }
}
