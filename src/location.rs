use std::path::{Path,PathBuf};

use crate::argument::Arguments;
use crate::program::SystemVersionDefaults;

#[derive(Debug)]
pub struct BaseLocation {
    pub system_version_path: PathBuf,
    pub dyld_shardcache_macos_path: PathBuf,
    pub dyld_shardcache_iphoneos_path: PathBuf,
}

impl BaseLocation {
    pub fn new(arguments: &Arguments) -> BaseLocation {     
        let system_version_path = Path::new("System/Library/CoreServices/SystemVersion");
        let dyld_shardcache_macos =  Path::new("System/Library/dyld");
        let dyld_shardcache_iphoneos = Path::new("System/Library/Caches/com.apple.dyld");

        BaseLocation {
            system_version_path: arguments.path_from_base(system_version_path),
            dyld_shardcache_macos_path: arguments.path_from_base(dyld_shardcache_macos),
            dyld_shardcache_iphoneos_path: arguments.path_from_base(dyld_shardcache_iphoneos),
        }
    }
}

#[derive(Debug)]
pub struct ResultsLocation {
    pub shared_cache_path: PathBuf,
    pub unique_version_path: PathBuf,
    pub temp_shared_cache_path: PathBuf
}

impl ResultsLocation {
    pub fn new(arguments: &Arguments, system_version: &SystemVersionDefaults) -> ResultsLocation {
        let version_folder = format!("{} ({})", system_version.product_version, system_version.product_build_version);
        let system_version = &system_version.product_name;
        let unique_version_path = arguments.path_from_results(Path::new(system_version.as_str())).join(version_folder);
        let shared_cache_path = unique_version_path.join("shared_cache");

        let temp_shared_cache_path = shared_cache_path.join("temp");

        ResultsLocation {
            shared_cache_path,
            unique_version_path,
            temp_shared_cache_path
        }
    }
}