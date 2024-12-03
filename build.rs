

fn main() {
    // Tell cargo to rebuild files that have changed
    println!("cargo::rerun-if-changed=external/dyld-shared-cache-extractor/dyld-shared-cache-extractor.cpp");
    
    cc::Build::new()
        .file("external/dyld-shared-cache-extractor/dyld-shared-cache-extractor.cpp")
        .compile("dyld-shared-cache-extractor");
}