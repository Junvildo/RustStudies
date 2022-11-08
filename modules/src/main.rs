use crate::utils::archive::arch::arch_file as arc;
mod utils;
use rand::Rng;
fn main() {
    utils::player::play_movie("snatch.mp4");
    utils::player::play_audio("rhcp.mp3");

    clean::perform_clean();

    clean::files::clean_files();

    arc("somefile.txt");

    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);
}

mod clean {
    pub fn perform_clean() {
        println!("Cleaning HDD");
    }

    pub mod files {
        pub fn clean_files() {
            println!("Removing unused files");
        }
    }
}
