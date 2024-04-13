mod path_manager;
mod dialogue_manager;

use crate::dialogue_manager::{ask_for_directories_dialogue, print_title};


fn main() {
    print_title();
    let directories = ask_for_directories_dialogue();
    println!("{} e {}", directories.0,directories.1)
}

