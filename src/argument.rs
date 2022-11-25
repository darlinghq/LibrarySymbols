use std::path::{Path,PathBuf};
use clap::{Parser};

#[derive(Debug)]
pub struct Arguments {
    pub results_path: String,
    pub base_path: String
}

#[derive(Parser)]
#[command(version, author = "Thomas A.", about = "Extracts library symbols from Apple's framework")]
struct RawArguments {
    /// The normal root directory in macOS, iOS, etc.
    /// If no argument is provided, the root path will be used.
    #[arg(long, value_name = "PATH")]
    standard_path: Option<String>,
    /// Where the symbols will be saved at.
    #[arg(value_name = "RESULT FOLDER")]
    results_path: String
}

impl Arguments {
    pub fn new() -> Arguments {
        let raw_arguments = RawArguments::parse();
        Self::into_arguments(raw_arguments)
    }

    fn into_arguments(raw_arguments: RawArguments) -> Arguments {
        let base_path = raw_arguments.standard_path.unwrap_or(String::from("/"));

        Arguments { 
            results_path: raw_arguments.results_path, 
            base_path
        }
    }

    pub fn path_from_base<P: AsRef<Path>>(&self, location: P) -> PathBuf {
        let base_path = Path::new(self.base_path.as_str());
        base_path.join(location)
    }

    pub fn path_from_results<P: AsRef<Path>>(&self, location: P) -> PathBuf {
        let results_path = Path::new(self.results_path.as_str());
        results_path.join(location)
    }
}