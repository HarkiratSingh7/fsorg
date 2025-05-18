use super::move_file_safely;
use log::error;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Result, Write},
    path::PathBuf,
};

pub struct Actions {
    /// actions (file, destination)
    actions: Vec<(PathBuf, PathBuf)>,

    /// Statistics
    pub total_files_scanned: u32,
    total_files_moved: u32,
    pub total_files_skipped: u32,
    pub total_files_errors: u32,
}

impl Actions {
    pub fn new() -> Self {
        Self {
            actions: vec![],
            total_files_scanned: 0,
            total_files_moved: 0,
            total_files_skipped: 0,
            total_files_errors: 0,
        }
    }

    pub fn from(file_name: &str) -> Self {
        let mut actions = Actions::new();

        match File::open(file_name) {
            Ok(file) => {
                for line in BufReader::new(file).lines().map_while(Result::ok) {
                    let line: Vec<&str> = line
                        .splitn(2, "->")
                        .map(|s| s.trim_matches([' ', '"']))
                        .collect();
                    actions.add_action(PathBuf::from(line[0]), PathBuf::from(line[1]));
                }
            }
            Err(err) => {
                error!(
                    "Fatal error occurred while opening plan {}: {}\nCannot proceed further with this plan !",
                    file_name, err
                );
            }
        }

        actions
    }

    /// Registers an action
    pub fn add_action(&mut self, source: PathBuf, destination: PathBuf) {
        self.actions.push((source, destination));
    }

    /// Executes the actions
    pub fn execute_actions(&mut self) {
        for (source_path, destination) in &self.actions {
            match move_file_safely(source_path.as_path(), destination.as_path()) {
                Ok(()) => {
                    // increment moved files counter
                    self.total_files_moved += 1;
                    println!(
                        "Moved file {} to {}",
                        source_path.display(),
                        destination.display()
                    );
                }
                Err(err) => {
                    // increment error counter
                    self.total_files_errors += 1;
                    error!(
                        "An error occurred while moving file {} to {}: {}",
                        source_path.display(),
                        destination.display(),
                        err
                    );
                }
            };
        }
    }

    /// Exports actions to a plain text file
    pub fn export_actions(&self, file_name: &str) -> Result<()> {
        println!("Writing action plan to file: {}", file_name);

        let mut file = fs::OpenOptions::new()
            .append(true)
            .create_new(true)
            .open(file_name)?;
        for (source, destination) in &self.actions {
            file.write_all(
                format!(
                    "\"{}\" -> \"{}\"\n",
                    source.display(),
                    destination.display()
                )
                .as_bytes(),
            )?;
        }

        Ok(())
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
}
