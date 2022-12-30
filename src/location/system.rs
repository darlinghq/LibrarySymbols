use std::{path::{PathBuf, Path}, fs::{read_dir, ReadDir}, collections::{VecDeque}};

use crate::{arguments::cli::CliArguments, program::DyldSharedCacheExtractor};

use super::ResultsLocation;

const SYSTEM_FOLDER_BLACKLIST: [&str; 5] = ["Cryptexes","Library","Developer","Applications","Volumes"];

// where we searched for the library files (either directly on the filesystem
// or extracted from a sharedcache container).
pub const STANDARD_DIR: &str = "standard";
pub const SHAREDCACHE_DIR: &str = "sharedcache";
#[derive(Debug)]
pub enum SymbolsLocationType {
    Standard,
    SharedCache,
    Unknown
}

// The filesystem for where we will start the search.
const ROOT_DIR: &str = "root";
const CRYPTEXES_DIR: &str = "cryptexes";
#[derive(Debug)]
pub enum VolumeType {
    Root,
    Cryptexes,
    Unknown
}

#[derive(Debug)]
pub struct SystemPathParser {
    pub save_path: PathBuf,
    pub symbol_folders: Vec<PathBuf>,
}

impl SystemPathParser {
    pub fn new(arguments: &CliArguments, results_location: &ResultsLocation) -> Vec<SystemPathParser> {
        let mut parser_paths: Vec<SystemPathParser> = Vec::new();

        parser_paths.append(&mut Self::new_from_filesystem(ROOT_DIR,results_location,&arguments.base_path));
        if let Some(cryptexes_os_path) = &arguments.cryptexes_os_path {
            parser_paths.append(&mut Self::new_from_filesystem(CRYPTEXES_DIR,results_location,cryptexes_os_path));
        }

        parser_paths
    }

    pub fn breakdown_save_path(&self) -> (SymbolsLocationType,VolumeType) {
        let mut components = self.save_path.components();

        let symbol_location = if let Some(symbol_location_path) = components.next() {
            let symbol_location_path = symbol_location_path.as_os_str();
            if symbol_location_path == STANDARD_DIR {
                SymbolsLocationType::Standard
            } else if symbol_location_path == SHAREDCACHE_DIR {
                SymbolsLocationType::SharedCache
            } else {
                SymbolsLocationType::Unknown
            }
        } else { SymbolsLocationType::Unknown };

        let volume = if let Some(volume_path) = components.next() {
            let volume_path = volume_path.as_os_str();
            if volume_path == ROOT_DIR {
                VolumeType::Root
            } else if volume_path == CRYPTEXES_DIR {
                VolumeType::Cryptexes
            } else {
                VolumeType::Unknown
            }
        } else { VolumeType::Unknown };

        (symbol_location,volume)
    }

    fn new_from_filesystem<P: AsRef<Path>>(filesystem_identifier: &str, results_location: &ResultsLocation, filesystem_path: &P) -> Vec<SystemPathParser> {
        let mut parser_paths: Vec<SystemPathParser> = Vec::new();
        let save_path = Path::new(STANDARD_DIR).join(filesystem_identifier);
        let mut paths_to_sharedcache = Self::create_system_path_parser(&mut parser_paths,filesystem_path,save_path);

        while !paths_to_sharedcache.is_empty() {
            if let Some(sharedcache_folder) = paths_to_sharedcache.pop_front() {
                
                if let Ok(relative_path) = sharedcache_folder.strip_prefix(&filesystem_path) {
                    let system_identifier = 
                        if relative_path.starts_with("System/DriverKit") {"driverkit"}
                        else if relative_path.starts_with("System/Library") {"library"}
                        else { panic!("Unexpected filepath {:?}", relative_path) };

                    let save_path = Path::new(SHAREDCACHE_DIR).join(filesystem_identifier).join(system_identifier);
                    let sharedcache_extracted_path = results_location.temp_path.join(save_path.as_os_str());
                    let dyld_shared_cache_extractor = DyldSharedCacheExtractor::new(&sharedcache_extracted_path, &sharedcache_folder);
        
                    for extracted_path in dyld_shared_cache_extractor.extracted_paths {
                        let save_path = save_path.join(extracted_path.as_path().file_name().unwrap());
                        let unexpected_sharedcache_locations = Self::create_system_path_parser(&mut parser_paths,&extracted_path,save_path);
                        assert!(unexpected_sharedcache_locations.is_empty(),"Unexpected sharedcache files");
                    }

                }

            }
        }

        parser_paths
    }

    fn create_system_path_parser<P: AsRef<Path>>(parser_paths: &mut Vec<SystemPathParser>, filesystem_path: &P, save_path: PathBuf) ->  VecDeque<PathBuf>{
        let mut paths_to_sharedcache: VecDeque<PathBuf> = VecDeque::new();
        
        let mut paths_to_analyse: VecDeque<PathBuf> = VecDeque::new();
        let mut symbol_folders: Vec<PathBuf> = Vec::new();
        paths_to_analyse.push_back(PathBuf::from(filesystem_path.as_ref()));
        while !paths_to_analyse.is_empty() {
            if let Some(path) = paths_to_analyse.pop_front() {
                Self::append_path_if_exists(&mut symbol_folders,&path,"usr/lib");

                let system_path = path.join("System");
                if system_path.exists() {
                    Self::append_path_if_exists(&mut symbol_folders,&system_path,"Library/Frameworks");
                    Self::append_path_if_exists(&mut symbol_folders,&system_path,"Library/PrivateFrameworks");
    
                    let system_folders = read_dir(&system_path).unwrap();
                    Self::analyse_system_subfolders(system_folders,&mut paths_to_analyse);

                    if let Some(sharedcache_folder) = Self::determine_sharedcache_folder(&system_path) {
                        paths_to_sharedcache.push_back(sharedcache_folder)
                    };
                }
            }
        }

        
        parser_paths.push(SystemPathParser { save_path, symbol_folders });

        return paths_to_sharedcache;
    }

    fn append_path_if_exists<P: AsRef<Path>>(list_of_path: &mut Vec<PathBuf>, path: &P, path_to_append: &str) {
        let evaluate_path = path.as_ref().join(path_to_append);
        if evaluate_path.exists() { list_of_path.push(evaluate_path) }
    }

    fn determine_sharedcache_folder<P: AsRef<Path>>(path: &P) -> Option<PathBuf> {
        let mut dyld_shardcache_path: Option<PathBuf> = None;
        let dyld_shardcache_macos =  path.as_ref().join(Path::new("Library/dyld"));
        let dyld_shardcache_iphoneos = path.as_ref().join(Path::new("Library/Caches/com.apple.dyld"));

        let mut found_paths: u8 = 0;
        found_paths += if dyld_shardcache_macos.exists() {1} else {0};
        found_paths += if dyld_shardcache_iphoneos.exists() {1} else {0};
        assert!(found_paths <= 1, "Both {dyld_shardcache_macos:?} and {dyld_shardcache_iphoneos:?} exist");

        if dyld_shardcache_macos.exists() {
            dyld_shardcache_path = Some(dyld_shardcache_macos)
        } else if dyld_shardcache_iphoneos.exists() {
            dyld_shardcache_path = Some(dyld_shardcache_iphoneos)
        }
        
        return dyld_shardcache_path;
    }

    fn analyse_system_subfolders(system_folders: ReadDir, paths_to_sharedcache: &mut VecDeque<PathBuf>) {
        for system_folder in system_folders {
            let system_folder = system_folder.unwrap();
            let folder_name = system_folder.file_name();
            let file_type = system_folder.file_type().unwrap();

            if !SYSTEM_FOLDER_BLACKLIST.contains(&folder_name.to_str().unwrap()) && file_type.is_dir() {
                paths_to_sharedcache.push_back(system_folder.path());
            }
        }
    }
}