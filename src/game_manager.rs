use std::fs;

pub enum Games {
    RomeIiTotalWar,
    AttilaTotalWar
}

pub struct Game {
    name: String,
    data_directories: String,
    user_script_directories: String,
    already_present_pack_files: Vec<String>,
    enabled_mods: Vec<String>
}

impl Game {
    fn read_all_available_mods_in_data_directory(&mut self)  {
        if let Ok(entries) = fs::read_dir(&self.data_directories) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().is_file() && entry.path().extension() == Some("pack".as_ref()) {
                        if let Some(file_name) = entry.file_name().to_str() {
                            if !self.already_present_pack_files.contains(&file_name.to_string()) {
                                &self.enabled_mods.push(file_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn apply_mods(&mut self) {
        println!("trying to read mods data directory");
        self.read_all_available_mods_in_data_directory();
        println!("Found the following mods");
        for enabled_mod in &self.enabled_mods {
            println!("{}", enabled_mod)
        }
    }
}

pub fn map_enum_to_struct(game_to_map: &Games, game_directories: (String, String)) -> Game {
    match game_to_map {
        Games::RomeIiTotalWar => Game {
            name: "Rome II Total War".to_string(),
            data_directories: game_directories.0,
            user_script_directories: game_directories.1,
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
            data_directories: game_directories.0,
            user_script_directories: game_directories.1,
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


