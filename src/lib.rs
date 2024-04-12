use std::{env, fs};
use std::fs::File;
use std::io::Write;

pub fn save_directories_paths_in_file(game_data_path: String, user_script_directory: String) {
    println!("Saving the directories path!");

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

        match File::create(&path) {
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
    else {
        println!("Failed to obtain the directory path.");
    }

}