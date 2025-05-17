use log::{debug, error, info, warn};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::path::PathBuf;

const CWD: &str = ".";

#[derive(Debug, Serialize, Deserialize)]
/// Configurations representation for our application
pub struct Configurations {
    /// The map represents <file name regex> -> <Directory for this file to be stored>
    rules: HashMap<String, String>,

    #[serde(skip)]
    compiled_rules: Vec<(Regex, String)>,

    /// Working directory path where we have files to be organized.
    /// Default value will be current working directory
    #[serde(skip)]
    working_directory: PathBuf,

    /// The destination directory where we have to place the files
    #[serde(skip)]
    destination_directory: PathBuf,

    /// Configurations file path
    /// Default value be a current working directory
    #[serde(skip)]
    configuration_file: PathBuf,

    /// Version for future usage
    version: String,
}

impl Configurations {
    pub fn new() -> Self {
        Configurations {
            rules: HashMap::new(),
            compiled_rules: vec![],
            working_directory: CWD.into(),
            destination_directory: CWD.into(),
            configuration_file: PathBuf::new(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub fn get_working_directory(&self) -> &PathBuf {
        &self.working_directory
    }

    pub fn set_working_directory(&mut self, directory: PathBuf) {
        self.working_directory = directory;
    }

    pub fn set_destination_directory(&mut self, directory: PathBuf) {
        self.destination_directory = directory;
    }

    fn compile_regexes(&mut self) {
        self.compiled_rules = self
            .rules
            .iter()
            .filter_map(|(pattern, destination)| {
                // Regex::new(r.0).ok().map(|regex| (regex, r.1.clone())
                match Regex::new(pattern) {
                    Ok(regex) => {
                        debug!("Compiled regex: {} -> {}", pattern, destination);
                        Some((regex, destination.clone()))
                    }
                    Err(err) => {
                        error!(
                            "Failed to compile the regex {} for {}: {}",
                            pattern, destination, err
                        );
                        None
                    }
                }
            })
            .collect();
    }

    pub fn retrieve_destination_directory(&self, file_name: &str) -> Option<PathBuf> {
        for (regex_str, destination) in &self.compiled_rules {
            if regex_str.is_match(file_name) {
                return Some(
                    Path::new(&self.destination_directory)
                        .join(destination.clone())
                        .to_path_buf(),
                );
            }
        }

        None
    }

    pub fn load_configurations(&mut self, config_file: PathBuf) {
        match File::open(&config_file) {
            Ok(fp) => {
                match serde_json::from_reader::<BufReader<File>, Configurations>(BufReader::new(fp))
                {
                    Ok(configurations) => {
                        // Move the file mapper to our file_mapper
                        self.rules = configurations.rules;
                    }
                    Err(err) => {
                        error!(
                            "Error occurred while reading from configurations file:{} !",
                            err
                        );
                        self.seed_configurations();
                        self.store_configurations();
                    }
                }
            }
            Err(_) => {
                self.seed_configurations();
                self.configuration_file = config_file;
                self.store_configurations();
            }
        }

        self.compile_regexes();
    }

    fn seed_configurations(&mut self) {
        warn!("Loading default configurations !");
        self.rules = HashMap::from([
            (
                r"(?i)^.*\.(jpg|jpeg|png|gif|bmp|webp|tiff?)$".to_string(),
                "Images".to_string(),
            ),
            (
                r"(?i)^.*\.(pdf|docx?|xlsx?|pptx?|odt|ods|txt|rtf|csv|md)$".to_string(),
                "Documents".to_string(),
            ),
        ]);
        debug!("Default configurations loaded: {:?}", self.rules);
    }

    fn store_configurations(&self) {
        info!("Writing configurations to file !");
        match File::create(&self.configuration_file) {
            Ok(mut fp) => match serde_json::to_string_pretty(&self) {
                Ok(config) => match fp.write_all(config.as_bytes()) {
                    Ok(()) => {}
                    Err(err) => error!("Unable to write configurations to file: {} !", err),
                },
                Err(err) => error!("Unable to serialize the configurations: {} !", err),
            },
            Err(err) => error!("Unable to open file for writing configurations: {} !", err),
        }
    }
}
