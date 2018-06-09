#![cfg(test)]

use nzsc2p::two_player_game::{
    NZSCTwoPlayerGame,
    WhichPlayer,
};
use super::phase_as_json;

#[test]
fn it_works() {
    let mut g = NZSCTwoPlayerGame::new();
    assert_eq!(phase_as_json(&g), "{\"phase\":\"CHARACTER_CHOOSING\",\"a\":{\"points\":0,\"waits\":4,\"character_streak\":{\"times\":0,\"character\":null},\"selected_character\":null},\"b\":{\"points\":0,\"waits\":4,\"character_streak\":{\"times\":0,\"character\":null},\"selected_character\":null}}");

    g.process_choice(WhichPlayer::PlayerA, "Ninja".to_string()).unwrap();
    assert_eq!(phase_as_json(&g), "{\"phase\":\"CHARACTER_CHOOSING\",\"a\":{\"points\":0,\"waits\":4,\"character_streak\":{\"times\":0,\"character\":null},\"selected_character\":\"Ninja\"},\"b\":{\"points\":0,\"waits\":4,\"character_streak\":{\"times\":0,\"character\":null},\"selected_character\":null}}");
    g.process_choice(WhichPlayer::PlayerB, "Ninja".to_string()).unwrap();
    assert_eq!(phase_as_json(&g), "{\"phase\":\"CHARACTER_CHOOSING\",\"a\":{\"points\":0,\"waits\":4,\"character_streak\":{\"times\":1,\"character\":\"Ninja\"},\"selected_character\":null},\"b\":{\"points\":0,\"waits\":4,\"character_streak\":{\"times\":1,\"character\":\"Ninja\"},\"selected_character\":null}}");

    g.process_choice(WhichPlayer::PlayerA, "Ninja".to_string()).unwrap();
    g.process_choice(WhichPlayer::PlayerB, "Clown".to_string()).unwrap();
    assert_eq!(phase_as_json(&g), "{\"phase\":\"BOOSTER_CHOOSING\",\"a\":{\"points\":0,\"waits\":4,\"character\":\"Ninja\",\"selected_booster\":null},\"b\":{\"points\":1,\"waits\":4,\"character\":\"Clown\",\"selected_booster\":null}}");
}
