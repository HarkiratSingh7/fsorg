mod organizer;
use log::error;
use organizer::engine::Engine;
use std::env;
use std::path::PathBuf;

fn main() {
    env_logger::init();

    let mut engine = Engine::new();

    let mut last_argument = String::new();
    let mut last_utilized = true;
    let arguments: Vec<String> = env::args().skip(1).collect();
    for argument in &arguments {
        if argument.starts_with("-") {
            last_argument = argument.clone();
            last_utilized = false;
        } else {
            match last_argument.as_str() {
                "--source-dir" | "-s" => engine.change_working_directory(PathBuf::from(argument)),
                "--destination-dir" | "-d" => {
                    engine.change_destination_directory(PathBuf::from(argument))
                }
                "--load-config-file" | "-c" => {
                    engine.change_configurations(PathBuf::from(argument))
                }
                _ => {
                    error!("Invalid argument: {}\n", argument);
                    usage();
                    std::process::exit(-1);
                }
            };
            last_utilized = true;
        }
    }

    if !last_utilized {
        error!("Incomplete argument provided: {}\n", arguments.join(" "));
        usage();
        std::process::exit(-1);
    }

    engine.load_configurations();
    engine.organize();
    println!("***");
    println!("Total files scanned: {}", engine.get_total_files_scanned());
    println!("Total files moved: {}", engine.get_total_files_moved());
    println!("Total files skipped: {}", engine.get_total_files_skipped());
    println!(
        "Total errors encountered: {}",
        engine.get_total_files_errors()
    );
}

fn usage() {
    println!(
        "fsorg [OPTIONS]\n\
              Organizes the files according to user's rules based on regex file name matching.\n\n\
              --load-config-file | -c\tLoads the config file having the rules for file organization.\n\
                                \t\t\tIf not provided then it generates a basic fsorg.json which user can modify.\n\n\
              --source-dir | -s      \tSource directory containing unorganised files. (By default it is current working directory).\n\n\
              --destination-dir | -d \tDestination where to place the organized files. (By default it is current working directory).\n\n"
    );
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
}
