mod arguments;
mod helper;
mod program;
mod symbols;

use arguments::{cli::CliArguments, library_symbols::LibrarySymbolsArguments};
use program::{save_output_to_file, NmLibrarySymbols, OtoolLibrarySymbols, WhoAmIUserName};
use symbols::location::{common::CommonSymbolLocations, macho_executables::MachoExecutableLocation, CryptexesOsLocation, RootFileSystemLocation, SymbolsGetter};

use std::path::Path;

fn main() {
    let library_symboles_args;
    {
        let cli_args = CliArguments::new();
        library_symboles_args = LibrarySymbolsArguments::new(cli_args);
    }
    
    let root_filesystem_location = RootFileSystemLocation::new(&library_symboles_args);
    let cryptexes_os_location = CryptexesOsLocation::new(&library_symboles_args);

    generate_symbols(&library_symboles_args, "root_filesystem", &root_filesystem_location);
    
    if let Some(cryptexes_os_location) = cryptexes_os_location {
        generate_symbols(&library_symboles_args, "cryptexes_os", &cryptexes_os_location);
    }
}


fn generate_symbols<S: SymbolsGetter>(args: &LibrarySymbolsArguments, folder_name: &str, locations: &S) {
    let output = args.get_os_symbol_path().join(folder_name);
    let whoami = WhoAmIUserName::new();
    
    {
        let output = output.join("standard");
        let common_symbol_locations = locations.get_root_symbols();
        gather_executables_and_save_output(&output, common_symbol_locations, &whoami);
    }
    
    {
        let common_location_list = locations.get_shared_cache_symbols().get_shared_cache_list();
        let output = output.join("sharedcache");
        
        for shared_cache_location in common_location_list {
            let output = output.join(&shared_cache_location.location_name);

            for shared_cache_arch in &shared_cache_location.shared_cachees {
                let output = output.join(&shared_cache_arch.shared_cache_name);
                let common_location = &shared_cache_arch.symbols;
                gather_executables_and_save_output(&output, common_location, &whoami);
            }
        }
    }
}


fn gather_executables_and_save_output(output: &Path, common_symbol_locations: &CommonSymbolLocations, whoami: &WhoAmIUserName) {        
    let starting_path = common_symbol_locations.get_starting_path();
    
    for symbol_path in common_symbol_locations.get_paths() {
        let located_library_executables = MachoExecutableLocation::new(symbol_path);
        
        for macho_lib in located_library_executables.get_paths() {
            println!("Gathering symbol data for '{}'", macho_lib.to_string_lossy());
            
            let Ok(relative_path) = macho_lib.strip_prefix(starting_path) else { continue };
            let output = output.join(relative_path);

            let nm_output = NmLibrarySymbols::new(macho_lib);
            let otool_output = OtoolLibrarySymbols::new(macho_lib, &whoami);
        
            save_output_to_file(&nm_output, &output.join("nm.txt"));
            save_output_to_file(&otool_output, &output.join("otool.txt"));
        }
    }
}
