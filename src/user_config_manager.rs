use std::{env, fs};
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::PathBuf;

// this function will return the path that represent the file that is saved in the mod manager path and in it is written the paths for mods
// of the game so that the user can use them again after a boot
fn get_user_mods_paths_document() -> Option<PathBuf> {
    let executable_path = match env::current_exe() {
        Ok(exe_path) => exe_path.parent().map(|p| p.to_path_buf()),
        Err(error) => {
            println!("Error obtaining current path of the CLI, error: {}", error);
            None
        }
    };

    if let Some(mut path) = executable_path {
        let path_saved_file_name = "saved_path.txt";
        path.push(path_saved_file_name);
        return Some(path)
    }
    None
}

pub fn save_directories_paths_in_file(game_data_path: String, user_script_directory: String) {
    println!("Saving the directories path!");
    if let Some(mods_paths_file) = get_user_mods_paths_document() {
        match File::create(&mods_paths_file) {
            Ok(mut file) => {
                if let Err(error) = file.write_all(game_data_path.as_bytes()) {
                    println!("Failed to write game data path in the file!! error: {}", error);
                }
                if let Err(error) = file.write_all((user_script_directory + "\n").as_bytes()) {
                    println!("Failed to write user script directory in the file!! error: {}", error);
                }
                println!("File successfully created and paths saved.");
            },
            Err(error) => println!("Failed to create path file!! error: {}", error),
        }
    }
}

pub fn find_if_mods_path_file_is_present() -> bool {
    if let Some(path) = get_user_mods_paths_document() {
        return File::open(path).is_ok()
    }
    false
}

pub fn find_mods_path_written_in_file() -> (String, String) {
    let path = get_user_mods_paths_document().expect("Path not found");
    let content = fs::read_to_string(path).expect("Error trying to open mods path file, try to create it again");
    let mut lines = content.lines();
    let first_line = lines.next().unwrap_or_default().to_string();
    let second_line = lines.next().unwrap_or_default().to_string();
    (first_line, second_line)
}