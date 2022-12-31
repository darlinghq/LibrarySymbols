use std::path::{Path,PathBuf};
use clap::{Parser};

#[derive(Debug)]
pub struct CliArguments {
    pub results_path: PathBuf,
    pub base_path: PathBuf,
    pub cryptexes_os_path: Option<PathBuf>,
}

#[derive(Parser)]
#[command(version, author = "Thomas A.", about = "Extracts library symbols from Apple's framework/libraries")]
struct RawArguments {
    /// The normal root directory in macOS, iOS, etc.
    /// If no argument is provided, the root path will be used.
    #[arg(long, value_name = "PATH")]
    standard_path: Option<String>,
    /// Path to cryptexes OS directory.
    /// If no argument is provided, the program will first check
    /// if "/System/Cryptexes/OS" exists, if it doesn't exist, the
    /// option will be ignored.
    #[arg(long, value_name = "PATH")]
    cryptexes_os_path: Option<String>,
    /// Where the symbols will be saved at.
    #[arg(value_name = "RESULT FOLDER")]
    results_path: String,
}

impl CliArguments {
    pub fn new() -> CliArguments {
        let raw_arguments = RawArguments::parse();
        Self::into_arguments(raw_arguments)
    }

    fn into_arguments(raw_arguments: RawArguments) -> CliArguments {
        let base_path = raw_arguments.standard_path.unwrap_or_else(|| {
            println!("Standard path not provided. Falling back to root directory ('/')");
            String::from("/")
        });

        let base_path_temp = PathBuf::from(base_path.as_str());
        let cryptexes_os_path = raw_arguments.cryptexes_os_path.or_else(|| {
            let cryptexes_os_alt_path =  Path::new("System/Cryptexes/OS");
            let temp_path = base_path_temp.join(cryptexes_os_alt_path);

            let temp_path_str = temp_path.to_str().unwrap();
            println!("Cryptexes OS path not provided. Checking if path '{}' exists", temp_path_str);
            if temp_path.exists() {
                println!("Found '{}' path", temp_path_str);
                Some(String::from(temp_path_str))
            } else {
                println!("Unable to find '{}' path", temp_path_str);
                None
            }
        });

        let results_path =  PathBuf::from(raw_arguments.results_path);
        let base_path = PathBuf::from(&base_path);
        let cryptexes_os_path = cryptexes_os_path.map(|path| { PathBuf::from(path) });

        CliArguments {
            results_path, 
            base_path,
            cryptexes_os_path,
        }
    }
}