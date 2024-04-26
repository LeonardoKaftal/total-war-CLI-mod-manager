use std::{env, fs, io};
use std::fs::File;
use std::io::{BufRead, Seek, SeekFrom, Write};
use std::path::PathBuf;
use termion::input::TermRead;
use crate::game_manager::{Game, Games};

// this function will return the path that represent the file that is saved in the mod manager path and in it is written the paths for mods
// of the game so that the user can use them again after a boot
fn get_user_config_path() -> Option<PathBuf> {
    let executable_path = match env::current_exe() {
        Ok(exe_path) => exe_path.parent().map(|p| p.to_path_buf()),
        Err(error) => panic!("Impossible to create USER CONFIG FILE, there was an error obtaining current executable path, error: {}", error)
    };

    if let Some(mut path) = executable_path {
        let user_config_file_name = "user_config.txt";
        path.push(user_config_file_name);
        return Some(path)
    }
    None
}

pub fn save_directories_in_config_file(game: &Game) {
    println!("Trying to save the game directories path in the user config file!");

    if let Some(mods_paths_file) = get_user_config_path() {
        if File::open(&mods_paths_file).is_err() {
             File::create(&mods_paths_file).expect("Failed to create user config file {}");
        }

        let mut file = eliminate_game_occurrences_from_config_file(&game, &mods_paths_file).expect("Impossible to eliminate old occurrences of game path in the config file,\
        try create it again");


        write_game_data_to_file(game, &mut file).expect("Failed to write game data to temporary file");
        println!("File successfully created and paths saved.");
    }
}

fn eliminate_game_occurrences_from_config_file(game: &Game, config_file_path: &PathBuf) -> io::Result<(File)> {
    let mut line_counter = 0;
    let mut buff: Vec<String> = Vec::new();

    for line in fs::read_to_string(config_file_path).unwrap().lines() {
        if line != game.name && line_counter == 0{
            buff.push(line.to_string())
        }
        else {
            line_counter += 1;
            if line_counter == 3 {
                line_counter = 0;
            }
        }
    }

    fs::remove_file(config_file_path)?;
    let mut config_file = File::create(config_file_path).expect("Impossible to create config file");
    buff.iter().for_each(|line|
        config_file.write_all(&format!("{}\n", line).as_bytes()).expect("impossible to write in the newly created config file")
    );
    Ok(config_file)
}

fn write_game_data_to_file(game: &Game, file: &mut File) -> io::Result<()> {
    let game_data_directory = game.data_directories.as_ref().expect("Failed to read game data path!!");
    let user_script_directory = game.user_script_directories.as_ref().expect("Failed to read user script directory!!!");

    file.write_all(&format!("{}\n", &game.name).as_bytes()).unwrap_or_else(|err|  panic!("Failed to write game title in the config file!! \
                Error: {}, impossible to save the paths in the config file:", err));
    file.write_all(format!("{}\n", game_data_directory).as_bytes()).unwrap_or_else(|err|  panic!("Failed to write game data path in the file!! \
                Error: {}, impossible to save the paths in the config file", err));
    file.write_all(format!("{}\n", user_script_directory).as_bytes()).unwrap_or_else(|err| panic!("Failed to write user script path in the file!! \
                Error: {}, impossible to save the paths in the config file", err));

    Ok(())
}

pub fn find_game_mods_paths_in_user_config_file(game: &Game) -> Option<(String, String)> {
    if let Some(path) = get_user_config_path() {
        match fs::read_to_string(&path) {
            Ok(contents) => {
                let mut config_lines = contents.lines().peekable();
                while let Some(line) = config_lines.next() {
                    if line == game.name {
                        return Some((
                            config_lines.next().expect("Error reading the lines of the config file, try generate it again").to_string(),
                            config_lines.next().expect("Error reading the lines of the config file, try generate it again").to_string(),
                        ));
                    }
                }
                None
            }
            Err(err) => {
                eprintln!("Error reading user config file {}: {}", path.display(), err);
                None
            }
        }
    } else {
        None
    }
}


