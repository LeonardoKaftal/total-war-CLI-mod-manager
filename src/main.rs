use std::{env, io};
use std::io::{stdout, Write};
use termion::{clear, color, cursor, style, terminal_size};
use total_war_CLI_mod_manager::save_directories_paths_in_file;


fn main() {
    print_title();
    let mut directories: (String,String) = (String::new(),String::new());
    if ()
        get_directories_path();
}

fn get_directories_path() -> (String, String) {
    let mut game_data_path = String::new();
    print!("Please insert the game data directory path!");
    println!();
    stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut game_data_path).expect("You must pass the data directory path of the game where all the game .pack \
    files are inserted, tw-mod-manager args1(game data path) args2(user script directory path)");
    stdout().flush().unwrap();

    print!("Please insert the user script directory path");
    println!();
    stdout().flush().unwrap();

    let mut user_script_directory = String::new();
    io::stdin().read_line(&mut user_script_directory).expect("You must pass the user script directory path of the game");
    stdout().flush().unwrap();

    println!("Directory correctly received, would you like to save them for the next boot?");
    stdout().flush().unwrap();

    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to parse the response");
    stdout().flush().unwrap();

    if response.to_ascii_lowercase().starts_with('y') {
        save_directories_paths_in_file(game_data_path.clone(),user_script_directory.clone())
    }
    (game_data_path, String::new())
}

fn print_title() {
    let title = "TOTAL WAR MOD MANAGER";

    let mut stdout = stdout();
    let mut handle = stdout.lock();

    if let Ok((width, _)) = terminal_size() {
        let padding = (width as usize - title.len()) / 2;
        let padded_title = format!("{:padding$}{}", "", title, padding = padding);
        writeln!(handle, "{}{}{}{}", color::Fg(color::Red), padded_title, color::Fg(color::Reset), style::Reset).unwrap();
    }
    else {
        writeln!(handle, "{}{}{}", color::Fg(color::Red), title, color::Fg(color::Reset)).unwrap();
    }

    stdout.flush().unwrap()
}
