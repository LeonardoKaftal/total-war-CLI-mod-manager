use std::io::{stdin, stdout, Write};
use crossterm::{ExecutableCommand, execute};
use crossterm::cursor::{MoveToColumn, SavePosition};
use crossterm::style::{Colors, Print, SetColors};
use crossterm::style::Color::{Black, Red, White};
use crossterm::terminal::size;
use crate::game_manager::{Game, Games};
use crate::user_config_manager::{find_game_mods_paths_in_user_config_file, save_directories_in_config_file};


pub fn print_red_string(text: &str) {
    execute!(
    stdout(),
        SetColors(Colors::new(Red, Black)),
        Print(format!("{}\n",text)),
        SetColors(Colors::new(White, Black))
    ).unwrap();
}



pub fn print_title() {
    let title = "TOTAL WAR MOD MANAGER";
    let terminal_width = size().unwrap().1 * 10 + (size().unwrap().1 / 2);
    let title_len = (title.len() * 2) as u16;
    let padding = (terminal_width - title_len) / 2;

    execute!(
        stdout(),
        SavePosition,
        SetColors(Colors::new(Red, Black)),
        MoveToColumn(padding),
        Print(format!("{}\n", title)),
        SetColors(Colors::new(White, Black))
    ).expect("Impossible");



    println!("ATTENTION, the mod manager will enable every mod it find in the data directory of the game, \
    so take out of the data folder every mod you would not like to enable");
    println!();
    println!("ATTENTION, every mod files name should be in lowercase, the mod manager in the process of applying the mod will \
    rename all of the mods to lowercase if found some in uppercase");
    println!();
    println!("ATTENTION, the mod manager only support .pack mod files, if it find some .BIN files (outdated mod format) then it will automatically convert to .pack file");
}


pub fn ask_for_game_directories_dialogue(chosen_game: &mut Game) -> &mut Game {
    print_red_string("\nWould you like to search in the config file if you have already saved the game path?");

    if user_prompt_yes() {
        return if let Some(mut paths) = find_game_mods_paths_in_user_config_file(chosen_game) {
            chosen_game.data_directories = Some(paths.0);
            chosen_game.user_script_directories = Some(paths.1);
            chosen_game
        } else {
            println!("No config file has been found");
            aks_for_directories_path(chosen_game)
        }
    }

    return aks_for_directories_path(chosen_game)
}



fn aks_for_directories_path(game: &mut Game) -> &mut Game {
    let mut game_data_path = String::new();
    print_red_string("Please insert the game data directory path!");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut game_data_path).expect("You must pass the data directory path of the game where all the game .pack \
    files are inserted, tw-mod-manager args1(game data path) args2(user script directory path)");
    stdout().flush().unwrap();

    print_red_string("Please insert the user script directory path");
    stdout().flush().unwrap();

    let mut user_script_directory = String::new();
    stdin().read_line(&mut user_script_directory).expect("You must pass the user script directory path of the game");
    stdout().flush().unwrap();

    print_red_string("Directory correctly received, would you like to save them in the config file? (YOU WILL NOT HAVE TO WRITE THEM AGAIN, \
    THEY WILL BE AUTOMATICALLY ASSOCIATED WITH THE GAME YOU SELECTED");
    stdout().flush().unwrap();
    // eliminate \n from the path with truncate()
    game_data_path.truncate(game_data_path.len() - 1);
    user_script_directory.truncate(user_script_directory.len() - 1);

    game.data_directories = Some(game_data_path);
    game.user_script_directories = Some(user_script_directory);

    if user_prompt_yes() {
        save_directories_in_config_file(game);
    }

    game
}

pub fn user_prompt_yes() -> bool {
    let mut response = String::new();
    println!("PLEASE insert only Y or N");
    stdin().read_line(&mut response).expect("Failed to parse the response");
    stdout().flush().unwrap();

    return response.to_ascii_lowercase().starts_with('y')
}

pub fn ask_user_what_games() -> Games {
    let mut response = String::new();
    print_red_string("What game do you want to enable the mod for?");
    println!("1: Attila Total War");
    println!("2: Rome II Total War");
    println!("3: Napoleon Total War");
    println!("4: Shogun II Total War");
    stdin().read_line(&mut response).expect("Impossible to read the input");
    stdout().flush().unwrap();

    return match response.as_str().trim() {
        "1" => Games::AttilaTotalWar,
        "2" => Games::RomeIITotalWar,
        _ => panic!("INVALID INPUT, IMPOSSIBLE TO READ THE GAME, please insert number 1 - 4")
    }
}

