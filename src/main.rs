mod user_config_manager;
mod dialogue_manager;
mod game_manager;

use crate::dialogue_manager::{ask_for_game_directories_dialogue, ask_user_what_games, print_title};
use crate::game_manager::map_enum_to_game;


fn main() {
    //print_title();
    let choose_game_enum = ask_user_what_games();
    let mut game = map_enum_to_game(&choose_game_enum, None,None);
    // there are two directories, one is the data directory that contains the .pack mods file and the other one is the user script which list all the activated mod
    ask_for_game_directories_dialogue(&mut game);
    game.apply_mods()
}

