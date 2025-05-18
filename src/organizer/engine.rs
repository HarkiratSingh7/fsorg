use super::actions::Actions;
use super::configurations::Configurations;
use super::{FAIL_CONFIG_FILE, get_home_dir};
use log::error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn get_config_file_path() -> Option<PathBuf> {
    get_home_dir().map(|d| d.join(".fsorg.json"))
}

/// Engine - The main file organiser module
pub struct Engine {
    /// Current configurations
    configurations: Configurations,

    /// Points to config file
    config_file: PathBuf,
}

impl Engine {
    /// Creates a basic engine instance
    pub fn new() -> Self {
        Self {
            configurations: Configurations::new(),
            config_file: match get_config_file_path() {
                Some(home_dir) => home_dir,
                None => PathBuf::from(FAIL_CONFIG_FILE),
            },
        }
    }

    /// Changes the source working directory, where we have to look for unorganised files.
    pub fn change_working_directory(&mut self, directory: PathBuf) {
        self.configurations.set_working_directory(directory);
    }

    /// Changes the destination directory, where we have to place the organised files.
    pub fn change_destination_directory(&mut self, directory: PathBuf) {
        self.configurations.set_destination_directory(directory);
    }

    /// Support for user provided custom configurations
    pub fn change_configurations(&mut self, config_file: PathBuf) {
        self.config_file = config_file;
    }

    /// Loads the currently set configurations into memory, and compiles the regular expressions.
    pub fn load_configurations(&mut self) {
        self.configurations
            .load_configurations(self.config_file.clone());
    }

    pub fn retrieve_rules(&self) -> Vec<(String, String)> {
        self.configurations.view_rules()
    }

    pub fn add_rule(&mut self, pattern: &str, destination: &str) {
        self.configurations.add_dynamic_rule(pattern, destination);
    }

    pub fn delete_rule(&mut self, pattern: &str) {
        self.configurations.delete_dynamic_rule(pattern);
    }

    /// Generates actions
    pub fn generate_actions(&self) -> Actions {
        let mut actions = Actions::new();
        if let Ok(absolute_path) = fs::canonicalize(self.configurations.get_working_directory()) {
            match fs::read_dir(self.configurations.get_working_directory()) {
                Ok(listings) => {
                    for listing in listings {
                        let entry = match listing {
                            Ok(e) => e.path(),
                            Err(err) => {
                                error!("{}", err);
                                continue;
                            }
                        };

                        if !entry.is_file() {
                            continue;
                        }

                        // increment total files counter
                        actions.total_files_scanned += 1;

                        let file_name = match entry.file_name().and_then(|n| n.to_str()) {
                            Some(name) => name,
                            None => {
                                // increment error counter
                                actions.total_files_errors += 1;
                                error!("Invalid file name: {}", entry.display());
                                continue;
                            }
                        };
                        if let Some(destination) = self
                            .configurations
                            .retrieve_destination_directory(file_name)
                        {
                            let destination = Path::new(&destination).join(file_name);
                            let absolute_file_path = absolute_path.join(file_name);
                            actions.add_action(absolute_file_path, destination);
                        } else {
                            // increment skipped files counter
                            actions.total_files_skipped += 1;
                        }
                    }
                }
                Err(err) => error!("Error occurred while listing directory entries: {}", err),
            }
        }

        actions
    }
}
