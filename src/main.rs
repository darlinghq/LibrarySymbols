mod argument;
mod location;
mod program;

fn main() {
    let arguments = argument::Arguments::new(std::env::args());
    let base_locations = location::BaseLocation::new(&arguments);

    let system_version = program::SystemVersionDefaults::new(base_locations.system_version_path.to_str().unwrap());
    let results_location = location::ResultsLocation::new(&arguments, &system_version);
    
    let dyld_shared_cache_extractor = program::DyldSharedCacheExtractor::new(&base_locations, &results_location);

    println!{"{:#?}",arguments}
    println!{"{:#?}",system_version}
    println!{"{:#?}",base_locations}
    println!{"{:#?}",results_location}
    println!{"{:#?}",dyld_shared_cache_extractor}
}
