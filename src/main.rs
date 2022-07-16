mod argument;
mod location;
mod program;

fn main() {
    let arguments = argument::Arguments::new(std::env::args());
    let mut locations = location::Location::new(&arguments);

    let system_version = program::SystemVersionDefaults::new(locations.system_version_path.to_str().unwrap());
    locations.initalize_unique_version_path(&arguments, &system_version);
    println!{"{:?}",system_version}
    println!{"{:?}",locations}
}
