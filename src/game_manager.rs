use std::{fs, io};
use std::fs::{DirEntry, File};
use std::io::{Seek, SeekFrom, stdin, Write};
use std::path::PathBuf;
use crate::dialogue_manager::print_red_string;

pub enum Games {
    RomeIITotalWar,
    AttilaTotalWar
}

pub struct Game {
    pub name: String,
    pub data_directories: Option<String>,
    pub user_script_directories: Option<String>,
    pub already_present_pack_files: Vec<String>,
    pub enabled_mods: Vec<String>
}

impl Game {
    fn get_user_script_file_path(&self) -> String {
        let user_script_directory = self.user_script_directories.as_ref().expect("ERROR: IMPOSSIBLE to read available mods because you have not set the game directories!!");
        let mut user_script_directory_buff = PathBuf::from(user_script_directory);
        user_script_directory_buff.push("user.script.txt");
        user_script_directory_buff.to_str().expect("Error trying to parse user script directory, try to insert it again").to_string()
    }

    fn read_all_available_mods_in_data_directory(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let data_directory = self.data_directories.clone().expect("ERROR: IMPOSSIBLE to read available mods because \
        you have not set the game data directory, insert it again").to_string();
        let entries = fs::read_dir(data_directory)?;

        for entry in entries {
            let entry = entry?;
            if entry.path().extension() == Some("pack".as_ref()) {
                if let Some(file_name) = entry.file_name().to_str() {
                    if !self.already_present_pack_files.contains(&file_name.to_string()) {
                        let new_file_name = Self::rename_file_to_lowercase_if_uppercase(&entry, file_name);
                        self.enabled_mods.push(new_file_name);
                    }
                }
            }
            else if entry.path().extension() == Some("bin".as_ref()) {
                if let Some(file_name) = entry.file_name().to_str() {
                    print_red_string(format!("WARNING, a mod with name: {} has the \
                    BIN mod formact that is outdated for the current game, the mod manager will try to convert it", file_name).as_str());
                    println!("Press enter to continue.......");
                    stdin().read_line(&mut String::new()).unwrap();
                    self.convert_bin_to_pack_file(entry)
                }
            }
        }

        Ok(())
    }

    fn rename_file_to_lowercase_if_uppercase(entry: &DirEntry, file_name: &str) -> String {
        let file_contain_uppercase_char = file_name.chars().any(|c| c.is_uppercase());
        if file_contain_uppercase_char {
            let mut old_path = entry.path();
            let new_file_name = file_name.to_ascii_lowercase();
            if old_path.extension() == Some("bin".as_ref()) || old_path.extension() == Some("BIN".as_ref()) {
                old_path = PathBuf::from(old_path.to_str().unwrap().replace(".BIN", ".pack").replace(".bin", ".pack"));
            }
            let mut new_path = old_path.clone();
            new_path.pop();
            new_path.push(&new_file_name);
            fs::rename(old_path, new_path).expect(&format!("Impossible to rename to lowercase mod {}, try to rename it manually to lowercase", file_name));
            return new_file_name
        }
        String::from(file_name)
    }

    // same process of using a hex editor
    fn convert_bin_to_pack_file(&mut self, file_entry: DirEntry) {
        let bin_file_path = file_entry.path();
        let pack_file_path = bin_file_path.with_extension("pack");
        let mut bin_file = File::open(&bin_file_path).expect("Impossible to open BIN mod file");
        let mut pack_file = File::create(&pack_file_path).expect("Impossible to create .pack file in the process of converting BIN file,ERROR");

        bin_file.seek(SeekFrom::Start(8)).expect("Impossible to eliminate first 7 offset of BIN file");
        io::copy(&mut bin_file, &mut pack_file).expect("Impossible to copy the content of the bin file to the pack file");

        // remove old BIN file
        fs::remove_file(file_entry.path()).unwrap();
        println!("File successfully converted");
        let mut new_file_name = String::from(file_entry.file_name().to_str().unwrap());

        // remove old extension and add .pack extesnion to the new name of the mod (in memory name, not the file name of the system, that has already been created
        new_file_name.truncate(new_file_name.len() - 4);
        new_file_name.push_str(".pack");
        new_file_name = Self::rename_file_to_lowercase_if_uppercase(&file_entry, new_file_name.clone().as_str());
        self.enabled_mods.push(new_file_name);
    }


    pub fn apply_mods(&mut self) {
        println!("Trying to read mods data directory");
        self.read_all_available_mods_in_data_directory().unwrap_or_else(|error| panic!("Error trying to read available mods: {}", error));
        println!("Applying the following mods");
        let user_script_file_path = self.get_user_script_file_path();
        let mut user_script_file = File::create(user_script_file_path).expect("Impossible to overwrite user script file in the process \
        of applying mods");
        for enabled_mod in &self.enabled_mods {
            println!("{}", enabled_mod);
            let mod_string = format!(r#"mod "{}";"#, enabled_mod);
            user_script_file.write_all(format!("{}\n",mod_string).as_bytes()).expect("Impossible to write enabled mods in the user script file!!!")
        }
    }
}


pub fn map_enum_to_game(game_to_map: &Games, game_data_directory: Option<String>, game_user_script_directory: Option<String>) -> Game {
    match game_to_map {
        Games::RomeIITotalWar => Game {
            name: "Rome II Total War".to_string(),
            data_directories: game_data_directory,
            user_script_directories: game_user_script_directory,
            already_present_pack_files: vec![
                "boot.pack", "data.pack", "data_rome2.pack", "local_en.pack", "local_en_rome2.pack",
                "models.pack", "models_rome2.pack", "models2.pack", "models2_rome2.pack", "movies.pack",
                "movies_rome2.pack", "music.pack", "music_en.pack", "music_en_rome2.pack", "music_rome2.pack",
                "sound.pack", "sound_rome2.pack", "terrain.pack", "terrain_rome2.pack", "terrain2.pack",
                "terrain2_rome2.pack", "tiles.pack", "tiles_rome2.pack", "tiles2.pack", "tiles2_rome2.pack",
                "tiles3.pack", "tiles3_rome2.pack", "tiles4.pack", "tiles4_rome2.pack", "blood_rome2.pack", "divided.pack",
                "gaul.pack", "greeks.pack", "invasion.pack", "local_en_shared_rome2.pack", "models3_rome2.pack", "music_en_shared_rome2.pack",
                "punic.pack", "terrain3_rome2.pack"
            ].iter().map(|s| s.to_string()).collect(),
            enabled_mods: vec![]
        },
        Games::AttilaTotalWar => Game {
            name: "Attila Total War".to_string(),
            data_directories: game_data_directory,
            user_script_directories: game_user_script_directory,
            already_present_pack_files: vec![
                "boot.pack", "data.pack", "local_en.pack", "local_en_shared_rome2.pack", "models.pack",
                "models2.pack", "models3.pack", "movies.pack", "music.pack", "music_en.pack",
                "music_en_shared_rome2.pack", "music_rome2.pack", "sound.pack", "terrain.pack",
                "terrain2.pack", "tiles.pack", "tiles2.pack", "tiles3.pack", "tiles4.pack", "belisarius.pack", "blood.pack",
                "charlemagne.pack", "slavs.pack"
            ].iter().map(|s| s.to_string()).collect(),
            enabled_mods: vec![]
        },
    }
}


