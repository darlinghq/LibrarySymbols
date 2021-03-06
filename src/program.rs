use std::{process::{Command, Output}, fs::{read_dir, ReadDir, File}, path::{Path, PathBuf}, io::Read};

use crate::location::{BaseLocation, ResultsLocation};

fn parse_stdout(output: Output) -> Vec<String> {
    let raw_string = String::from_utf8(output.stdout).expect("Unable to save output");
    
    let mut vec: Vec<String> = Vec::new();
    for line in raw_string.lines() {
        vec.push(line.to_string());
    }

    vec
}


const PRODUCT_BUILD_VERSION: &str = "ProductBuildVersion";
const PRODUCT_NAME: &str = "ProductName";
const PRODUCT_VERSION: &str = "ProductVersion";
#[derive(Debug)]
pub struct SystemVersionDefaults {
    pub product_build_version: String,
    pub product_name: String,
    pub product_version: String
}

impl SystemVersionDefaults {
    pub fn new(path: &str) -> SystemVersionDefaults {
        let product_build_version = SystemVersionDefaults::launch_program(path, PRODUCT_BUILD_VERSION);
        let product_name = SystemVersionDefaults::launch_program(path, PRODUCT_NAME);
        let product_version = SystemVersionDefaults::launch_program(path, PRODUCT_VERSION);

        SystemVersionDefaults {
            product_build_version,
            product_name,
            product_version
        }
    }

    fn launch_program(path: &str, key: &str) -> String {
        let output = Command::new("defaults")
            .args(["read", path, key])
            .output()
            .expect("Unable to launch 'defaults' application");

        parse_stdout(output).first().expect("Unable to obtain value").to_string()
    }
}

#[derive(Debug)]
pub struct DyldSharedCacheExtractor {
    pub extracted_paths: Vec<PathBuf>
}

impl DyldSharedCacheExtractor {
    pub fn new(base_location: &BaseLocation, results_location: &ResultsLocation) -> DyldSharedCacheExtractor {
        let mut instance =  DyldSharedCacheExtractor {
            extracted_paths: Vec::new()
        };
        
        if let Ok(read_dir) = read_dir(&base_location.dyld_shardcache_macos_path) {
            println!("Inspecting {:?} for shared cache", base_location.dyld_shardcache_macos_path);
            instance.extract_shared_library(results_location,read_dir);
        }

        if let Ok(read_dir) = read_dir(&base_location.dyld_shardcache_iphoneos_path) {
            println!("Inspecting {:?} for shared cache", base_location.dyld_shardcache_iphoneos_path);
            instance.extract_shared_library(results_location,read_dir);
        }

        instance
    }

    fn extract_shared_library(&mut self, results_location: &ResultsLocation, read_dir: ReadDir) {
        for dir_entry in read_dir {
            let dir_entry = dir_entry.unwrap();
            let file_path = dir_entry.path();

            if self.is_shared_cache_file(file_path.as_path()) {
                let filename = file_path.as_path().file_name().expect("Unable to get file name");
                let temp_path = results_location.temp_shared_cache_path.join(filename);
                DyldSharedCacheExtractor::launch_program(file_path.as_path(), &temp_path);

                // If the path doesn't exist after `dyld-shared-cache-extractor` finishes executing, it means that
                // the application was not able to extract anything from it.
                if temp_path.is_dir() {
                    self.extracted_paths.push(temp_path);
                }
            }
        }
    }

    fn is_shared_cache_file(&self, file_path: &Path) -> bool {
        if file_path.is_file() {
            if let Ok(mut file) = File::open(file_path) {
                let mut magic: [u8; 5] = [0; 5];
                if let Ok(_) = file.read(&mut magic) {
                    return [b'd',b'y',b'l',b'd',b'_'] == magic;
                }
            }
        }

        return false
    }

    fn launch_program(shared_cache_path: &Path, temp_path: &Path) {
        let _ = Command::new("dyld-shared-cache-extractor")
            .args([shared_cache_path, temp_path])
            .status()
            .expect("Unable to launch 'dyld-shared-cache-extractor' application");
    }
}


#[derive(Debug)]
pub struct NmLibrarySymbols {
    pub raw_output: String
}

impl NmLibrarySymbols {
    pub fn new<P: AsRef<Path>>(macho_path: P) -> NmLibrarySymbols {
        let raw_output = NmLibrarySymbols::launch_program(macho_path);
        
        NmLibrarySymbols {
            raw_output
        }
    }

    fn launch_program<P: AsRef<Path>>(macho_path: P) -> String {
        let output = Command::new("nm")
        .args(["-m", macho_path.as_ref().to_str().expect("Unable to convert path to string")])
        .output()
        .expect("Unable to launch 'nm' application");

        String::from_utf8(output.stdout).expect("Unable to save output")
    }
}

#[derive(Debug)]
pub struct OtoolLibrarySymbols {
    pub raw_output: String
}

impl OtoolLibrarySymbols {
    pub fn new<P: AsRef<Path>>(macho_path: P, whoami: &WhoAmIUserName) -> OtoolLibrarySymbols {
        let raw_output = OtoolLibrarySymbols::launch_program(macho_path,whoami);

        OtoolLibrarySymbols {
            raw_output
        }
    }

    fn launch_program<P: AsRef<Path>>(macho_path: P, whoami: &WhoAmIUserName) -> String {
        let output = Command::new("otool")
        .args(["-L", macho_path.as_ref().to_str().expect("Unable to convert path to string")])
        .output()
        .expect("Unable to launch 'otool' application");

        OtoolLibrarySymbols::mask_user_account(String::from_utf8(output.stdout).expect("Unable to save output"),whoami)
    }

    fn mask_user_account(value: String, whoami: &WhoAmIUserName) -> String {
        value.replace(whoami.macos_users_dir.as_str(), "/Users/[Removed Username]")
    }
}

#[derive(Debug)]
pub struct WhoAmIUserName {
    pub username: String,
    pub macos_users_dir: String
}

impl WhoAmIUserName {
    pub fn new() -> WhoAmIUserName {
        let username = WhoAmIUserName::launch_program();
        let macos_users_dir = format!("/Users/{}",username);

        WhoAmIUserName {
            username,
            macos_users_dir
        }
    }

    fn launch_program() -> String {
        let output = Command::new("whoami")
        .output()
        .expect("Unable to launch 'whoami' application");

        parse_stdout(output).first().expect("Unable to obtain value").to_string()
    }
}