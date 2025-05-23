use rfd::FileDialog;
use std::env;
use std::path::PathBuf;

fn save_file() -> Option<PathBuf> {
    FileDialog::new().set_title("Save File").save_file()
}

fn set_directory() -> Option<PathBuf> {
    FileDialog::new().set_title("Set Directory").pick_folder()
}

fn load_file() -> Option<PathBuf> {
    FileDialog::new().set_title("Load file").pick_file()
}

fn help() {
    println!("Usage:dialogtocli <command>");
    println!("Commands:");
    println!("  -h   Show this help message");
    println!("  -s   Save a file");
    println!("  -l   Load a file");
    println!("  -d   Set a directory");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let path = match args[1].as_str() {
        "-h" => {
            help();
            return;
        }
        "-s" => save_file(),
        "-l" => load_file(),
        "-d" => set_directory(),
        _ => {
            println!("Unknown command: {}", args[1]);
            help();
            return;
        }
    };

    if let Some(path) = path {
        println!("{}", path.display());
    } else {
        panic!("No file selected");
    }
}
