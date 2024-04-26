use std::fmt::{Display, Error, Formatter};
use std::fs;
use std::path::PathBuf;

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
    fn get_user_script_directory(&self) -> String {
        let data_directory_path = self.data_directories.as_ref().expect("ERROR: IMPOSSIBLE to read available mods because you have not set the game directories!!");
        let mut data_directory_buff = PathBuf::from(data_directory_path);
        data_directory_buff.push("user.script.txt");
        data_directory_buff.to_str().expect("Error trying to parse user script directory, try to insert it again").to_string()
    }

    fn read_all_available_mods_in_data_directory(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let data_directory = self.data_directories.clone().expect("ERROR: IMPOSSIBLE to read available mods because \
        you have not set the game data directory, insert it again").to_string();
        let entries = fs::read_dir(data_directory)?;

        for entry in entries {
            let entry = entry?;
            if entry.path().is_file() && entry.path().extension() == Some("pack".as_ref()) {
                if let Some(file_name) = entry.file_name().to_str() {
                    if !self.already_present_pack_files.contains(&file_name.to_string()) {
                        self.enabled_mods.push(file_name.to_string());
                    }
                }
            }
        }

        Ok(())
    }


    pub fn apply_mods(&mut self) {
        println!("Trying to read mods data directory");
        self.read_all_available_mods_in_data_directory().unwrap_or_else(|error| panic!("Error trying to read available mods: {}", error));
        println!("Found the following mods");
        for enabled_mod in &self.enabled_mods {
            println!("{}", enabled_mod)
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
                "tiles3.pack", "tiles3_rome2.pack", "tiles4.pack", "tiles4_rome2.pack"
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


