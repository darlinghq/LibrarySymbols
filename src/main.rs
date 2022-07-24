mod argument;
mod clean;
mod location;
mod program;
mod symbols;

use std::path::Path;

fn main() {
    let arguments = argument::Arguments::new(std::env::args());
    let base_locations = location::BaseLocation::new(&arguments);

    let system_version = program::SystemVersionDefaults::new(base_locations.system_version_path.to_str().unwrap());
    let results_location = location::ResultsLocation::new(&arguments, &system_version);
    let whoami_username = program::WhoAmIUserName::new();

    clean::Cleanup::remove_saved_symbols(&results_location);

    let dyld_shared_cache_extractor = program::DyldSharedCacheExtractor::new(&base_locations, &results_location);
    
    for path in dyld_shared_cache_extractor.extracted_paths.iter() {
        let parse_filesystem_symbols = symbols::ParseBaseFilesystem::new(path);
        let shared_cache_folder = path.file_name().expect("Unable to obtain shared cache folder name").to_str();
        parse_filesystem_symbols.traverse(&results_location.shared_cache_path, shared_cache_folder, path, &whoami_username);
    }

    clean::Cleanup::remove_temp(&results_location);

    {
        let path = Path::new(arguments.base_path.as_str());
        let parse_filesystem_symbols = symbols::ParseBaseFilesystem::new(path);
        parse_filesystem_symbols.traverse(&results_location.unique_version_path, Some("standard"), path, &whoami_username);
    }
    
    clean::Cleanup::remove_temp(&results_location);

    println!{"{:#?}",arguments}
    println!{"{:#?}",system_version}
    println!{"{:#?}",base_locations}
    println!{"{:#?}",results_location}
    println!{"{:#?}",dyld_shared_cache_extractor}
    println!("{:#?}",whoami_username)
}
