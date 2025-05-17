mod organizer;
use log::error;
use organizer::engine::Engine;
use std::env;
use std::path::PathBuf;

enum Action<'a> {
    AddRule(&'a str, &'a str),
    DeleteRule(&'a str),
    ViewRule,
    Organise,
}

fn main() {
    env_logger::init();

    let mut engine = Engine::new();
    let mut action = Action::Organise;

    let mut last_argument = String::new();
    let mut last_utilized = true;
    let arguments: Vec<String> = env::args().skip(1).collect();
    for argument in &arguments {
        match argument.as_str() {
            "-v" | "--view-rules" => action = Action::ViewRule,
            "--help" | "-h" | "?" | "-?" => {
                usage();
                std::process::exit(0);
            }
            _ => {
                if argument.starts_with("-") {
                    last_argument = argument.clone();
                    last_utilized = false;
                } else {
                    match last_argument.as_str() {
                        "--source-dir" | "-s" => {
                            engine.change_working_directory(PathBuf::from(argument))
                        }
                        "--destination-dir" | "-d" => {
                            engine.change_destination_directory(PathBuf::from(argument))
                        }
                        "--config" | "-c" => engine.change_configurations(PathBuf::from(argument)),
                        "-a" | "--add-rule" => {
                            let rule_input: Vec<&str> = argument
                                .splitn(2, ",")
                                .map(|s| s.trim_matches([' ', '"']))
                                .collect();
                            if rule_input.len() != 2 {
                                error!("Invalid syntax for adding a rule !");
                                usage();
                                std::process::exit(-1);
                            }

                            action = Action::AddRule(rule_input[0], rule_input[1]);
                        }
                        "-r" | "--remove-rule" => {
                            action = Action::DeleteRule(argument.trim_matches([' ', '"']))
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
        }
    }

    if !last_utilized {
        error!("Incomplete argument provided: {}\n", arguments.join(" "));
        usage();
        std::process::exit(-1);
    }

    engine.load_configurations();
    match action {
        Action::Organise => {
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
        Action::ViewRule => only_print_rules(&engine),
        Action::AddRule(pattern, destination) => engine.add_rule(pattern, destination),
        Action::DeleteRule(pattern) => engine.delete_rule(pattern),
    }
}

fn usage() {
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    let left_width = 30;
    println!("Syntax: {:>10}", "fsorg [OPTIONS]");
    println!(
        "{:>left_width$} Specify the json config file. By default the config is present at ~/.fsorg.json",
        "--config | -c"
    );
    println!(
        "{:>left_width$} Source directory containing unorganised files. (By default it is current working directory).",
        "--source-dir | -s"
    );
    println!(
        "{:>left_width$} Destination where to place the organized files. (By default it is current working directory).",
        "--destination-dir | -d"
    );
    println!(
        "{:>left_width$} Adds a rule: fsorg -a \"(?i)^.*\\.(mp3|wav|ogg|flac)$\", \"Videos\"",
        "--add-rule | -a"
    );
    println!(
        "{:>left_width$} Removes a rule: fsorg -r \"(?i)^.*\\.(mp3|wav|ogg|flac)$\"",
        "--remove-rule | -r"
    );
    println!(
        "{:>left_width$} Views the current rules present in specified or default configs.",
        "--view-rules | -v"
    );
}

fn only_print_rules(engine: &Engine) {
    let mut width_pat = 0;
    let mut width_dest = 0;
    for (pat, dest) in engine.retrieve_rules() {
        width_pat = if width_pat < pat.len() {
            pat.len()
        } else {
            width_pat
        };
        width_dest = if width_dest < dest.len() {
            dest.len()
        } else {
            width_dest
        };
    }

    println!("+{:-<width_pat$} | {:->width_dest$}+", "", "");
    println!(
        "|{:<width_pat$} | {:>width_dest$}|",
        "Regex", "Destinations"
    );
    for (pattern, destination) in engine.retrieve_rules() {
        println!("+{:-<width_pat$} | {:->width_dest$}+", "", "");
        println!("|{:<width_pat$} | {:>width_dest$}|", pattern, destination);
    }

    println!("+{:-<width_pat$} | {:->width_dest$}+", "", "");

    std::process::exit(0);
}
