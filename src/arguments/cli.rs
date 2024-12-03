use std::path::{Path,PathBuf};
use clap::Parser;

#[derive(Debug)]
pub struct CliArguments {
    pub root_path: PathBuf,
    pub base_path: PathBuf,
    pub cryptexes_os_path: Option<PathBuf>,
}

#[derive(Parser)]
#[command(version, author = "Thomas A.", about = "Extracts library symbols from Apple's framework/libraries")]
struct RawArguments {
    /// The normal root directory in macOS, iOS, etc.
    /// If no argument is provided, the root path will be used.
    #[arg(long, value_name = "PATH")]
    root_path: Option<String>,
    /// Path to cryptexes OS directory.
    /// If no argument is provided, the program will first check
    /// if "/System/Cryptexes/OS" exists, if it doesn't exist, the
    /// option will be ignored.
    #[arg(long, value_name = "PATH")]
    cryptexes_os_path: Option<String>,
    /// Where the symbols will be saved at.
    #[arg(value_name = "RESULT FOLDER")]
    output_path: String,
}

impl CliArguments {
    pub fn new() -> CliArguments {
        let raw_arguments = RawArguments::parse();
        Self::into_arguments(raw_arguments)
    }

    fn into_arguments(raw_arguments: RawArguments) -> CliArguments {
        let cryptexes_os_string = raw_arguments.cryptexes_os_path;
        let base_path = raw_arguments.root_path.unwrap_or_else(|| {
            println!("Standard path not provided. Falling back to root directory ('/')");
            String::from("/")
        });

        let root_path =  PathBuf::from(raw_arguments.output_path);
        let base_path = PathBuf::from(&base_path);
        let cryptexes_os_path = Self::get_cryptexes_os_path(cryptexes_os_string, &base_path);

        CliArguments {
            root_path, 
            base_path,
            cryptexes_os_path,
        }
    }

    fn get_cryptexes_os_path(cryptexes_os_string: Option<String>, base_path: &PathBuf) -> Option<PathBuf> {
        let mut result = None;

        if let Some(cryptexes_os_string) = cryptexes_os_string {
            let potental_path = Path::new(cryptexes_os_string.as_str()).to_path_buf();
            if potental_path.is_dir() {
                result = Some(potental_path);
            }
        } else {
            println!("Cryptexes OS path not provided. Checking for Cryptexes OS from root directory");
            let default_potential_path = base_path.join("System/Cryptexes/OS");
            if default_potential_path.is_dir() {
                result = Some(default_potential_path);
            }
        }

        result
    }
}