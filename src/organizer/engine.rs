use super::configurations::Configurations;
use super::move_file_safely;
use log::error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

const DEFAULT_CONFIG_FILE: &str = "fsorg.json";

/// Engine - The main file organiser module
pub struct Engine {
    /// Current configurations
    configurations: Configurations,

    /// Points to config file
    config_file: PathBuf,

    /// Statistics
    total_files_scanned: u32,
    total_files_moved: u32,
    total_files_skipped: u32,
    total_files_errors: u32,
}

impl Engine {
    /// Creates a basic engine instance
    pub fn new() -> Self {
        Self {
            configurations: Configurations::new(),
            config_file: PathBuf::from(DEFAULT_CONFIG_FILE),
            total_files_scanned: 0,
            total_files_moved: 0,
            total_files_skipped: 0,
            total_files_errors: 0,
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

    pub fn get_total_files_scanned(&self) -> u32 {
        self.total_files_scanned
    }

    pub fn get_total_files_moved(&self) -> u32 {
        self.total_files_moved
    }

    pub fn get_total_files_skipped(&self) -> u32 {
        self.total_files_skipped
    }

    pub fn get_total_files_errors(&self) -> u32 {
        self.total_files_errors
    }

    /// Organizes the files based on the configurations loaded
    pub fn organize(&mut self) {
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
                        self.total_files_scanned += 1;

                        let file_name = match entry.file_name().and_then(|n| n.to_str()) {
                            Some(name) => name,
                            None => {
                                // increment error counter
                                self.total_files_errors += 1;
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
                            match move_file_safely(absolute_file_path.as_path(), &destination) {
                                Ok(()) => {
                                    // increment moved files counter
                                    self.total_files_moved += 1;
                                    println!(
                                        "Moved file {} to {}",
                                        file_name,
                                        destination.display()
                                    );
                                }
                                Err(err) => {
                                    // increment error counter
                                    self.total_files_errors += 1;
                                    error!(
                                        "An error occurred while moving file {} to {}: {}",
                                        file_name,
                                        destination.display(),
                                        err
                                    );
                                }
                            };
                        } else {
                            // increment skipped files counter
                            self.total_files_skipped += 1;
                        }
                    }
                }
                Err(err) => error!("Error occurred while listing directory entries: {}", err),
            }
        }
    }
}
