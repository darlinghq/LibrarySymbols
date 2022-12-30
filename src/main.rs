mod arguments;
mod clean;
mod location;
mod program;
mod symbols;

use std::{path::{Path, PathBuf}, fs::{File, create_dir_all}, io::Write};

use location::system::SystemPathParser;

#[derive(Debug)]
struct GlobalMainVariables {
    arguments: arguments::cli::CliArguments,
    system_path_parser: Vec<location::system::SystemPathParser>,
    results_location: location::ResultsLocation,
    whoami_username: program::WhoAmIUserName,
}

impl GlobalMainVariables {
    pub fn new() -> GlobalMainVariables {
        let arguments = arguments::cli::CliArguments::new();
        let system_version_path = arguments.base_path.join(Path::new("System/Library/CoreServices/SystemVersion.plist"));

        assert!(system_version_path.exists(), "Unable to find SystemVersion.plist");
        
        let system_version = program::SystemVersionDefaults::new(system_version_path.to_str().unwrap());
        let whoami_username = program::WhoAmIUserName::new();
        let results_location = location::ResultsLocation::new(&arguments, system_version);

        clean::remove_saved_symbols(&results_location);

        let system_path_parser = location::system::SystemPathParser::new(&arguments, &results_location);

        GlobalMainVariables { 
            arguments,
            system_path_parser,
            results_location,
            whoami_username,
        }
    }

    pub fn parse_for_symbols(&self) {
        for path_to_parse in &self.system_path_parser {
            let filesystem_path = Self::determine_filesystem_path(&self, path_to_parse);
            for symbol_folder in &path_to_parse.symbol_folders {
                let macho_executables = location::walk_directory(symbol_folder, symbols::macho::is_file_macho);
                for macho_executable in macho_executables {
                    let relative_path = macho_executable.strip_prefix(&filesystem_path).unwrap().parent().unwrap();
                    let macho_executable_file_name = format!{"{}.symboldir",macho_executable.file_name().unwrap().to_str().unwrap()};
                    let macho_executable_dir = self.results_location.os_version_path.join(&path_to_parse.save_path).join(relative_path).join(macho_executable_file_name);
                    create_dir_all(&macho_executable_dir).unwrap();

                    let nm_program = program::NmLibrarySymbols::new(&macho_executable);
                    let mut nm_output_file = File::create(macho_executable_dir.join("nm.txt")).unwrap();
                    nm_output_file.write_all(nm_program.raw_output.as_bytes()).expect("Unable to save `nm` output");

                    let otool_program = program::OtoolLibrarySymbols::new(&macho_executable, &self.whoami_username);
                    let mut otool_output_file = File::create(macho_executable_dir.join("otool.txt")).unwrap();
                    otool_output_file.write_all(otool_program.raw_output.as_bytes()).expect("Unable to save `otool` output");
                }
            }
        }
    }

    fn determine_filesystem_path(&self, path_to_parse: &SystemPathParser) -> PathBuf{
        let (symbol_location,volume) = path_to_parse.breakdown_save_path();

        let arguments = &self.arguments;
        let results_location = &self.results_location;
        let starting_path = match symbol_location {
            location::system::SymbolsLocationType::Standard => {
                match volume {
                    location::system::VolumeType::Root => { arguments.base_path.clone() }
                    location::system::VolumeType::Cryptexes => { arguments.cryptexes_os_path.clone().unwrap() }
                    location::system::VolumeType::Unknown => { panic!("Unknown volumn location detected"); }
                }
            }
            location::system::SymbolsLocationType::SharedCache => { 
                results_location.temp_path.join(&path_to_parse.save_path)
            }
            location::system::SymbolsLocationType::Unknown => { panic!("Unknown symbol location detected"); }
        };

        starting_path
    }

    pub fn clean(&self) {
        clean::remove_temp(&self.results_location);
    }
}

fn main() {
    let execution: GlobalMainVariables = GlobalMainVariables::new();
    execution.parse_for_symbols();
    execution.clean();
}