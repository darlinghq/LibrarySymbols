mod argument;
mod clean;
mod location;
mod program;
mod symbols;

use std::path::Path;

fn analyse_system_path<P: AsRef<Path>>(path: &P, results_location: &location::ResultsLocation, whoami_username: &program::WhoAmIUserName, unique_folder :Option<&str>) {
    let parse_filesystem_symbols_list = symbols::ParseBaseFilesystem::new(path);
    for parse_filesystem_symbols in parse_filesystem_symbols_list {
        
        parse_filesystem_symbols.traverse(&results_location.shared_cache_path, unique_folder, path, &whoami_username);
    }
}

fn main() {
    let arguments = argument::Arguments::new(std::env::args());
    let base_locations = location::BaseLocation::new(&arguments);

    let system_version = program::SystemVersionDefaults::new(base_locations.system_version_path.to_str().unwrap());
    let results_location = location::ResultsLocation::new(&arguments, &system_version);
    let whoami_username = program::WhoAmIUserName::new();

    clean::Cleanup::remove_saved_symbols(&results_location);

    let dyld_shared_cache_extractor = program::DyldSharedCacheExtractor::new(&base_locations, &results_location);
    
    for path in dyld_shared_cache_extractor.extracted_paths.iter() {
        let shared_cache_folder = path.as_path().file_name().expect("Unable to obtain shared cache folder name").to_str();
        analyse_system_path(path,&results_location,&whoami_username,shared_cache_folder);
    }
    clean::Cleanup::remove_temp(&results_location);

    {
        let path = Path::new(arguments.base_path.as_str());
        let unique_folder = Some("standard");
        analyse_system_path(&path,&results_location,&whoami_username,unique_folder);
        clean::Cleanup::remove_temp(&results_location);
    }


    println!{"{:#?}",arguments}
    println!{"{:#?}",system_version}
    println!{"{:#?}",base_locations}
    println!{"{:#?}",results_location}
    println!{"{:#?}",dyld_shared_cache_extractor}
    println!("{:#?}",whoami_username)
}
