mod user_config_manager;
mod dialogue_manager;
mod game_manager;

use crate::dialogue_manager::{ask_for_directories_dialogue, ask_user_what_games, print_title};
use crate::game_manager::map_enum_to_struct;


fn main() {
    print_title();
    println!("ATTENTION, the mod manager will enable every mod it find in the data directory of the game, \
    so take out of the data folder every mod you would not like to enable");
    println!();
    println!("ATTENTION, every mod files name should be in lowercase, the mod manager in the process of applying the mod will \
    rename all of the mods to lowercase if found some in uppercase");
    let directories = ask_for_directories_dialogue();
    let mut game = map_enum_to_struct(&ask_user_what_games(), directories);
    game.apply_mods()
}

