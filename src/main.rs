fn main() {
    let game_data_path = std::env::args().nth(1).expect("You must pass the data directory path of the game where all the game .pack \
    files are inserted, tw-mod-manager args1(game data path) args2(user script directory path)");
    let user_script_directory = std::env::args().nth(2).expect("You must pass the user script directory path of the game");
    println!("                                                          TW MOD MANAGER CLI                                                       " );
    println!("Getting all the .pack files of the selected game directory")
}
