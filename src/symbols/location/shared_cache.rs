use std::path::Path;

use crate::program::DyldSharedCacheExtractor;
use crate::symbols::location::common::{CommonSymbolLocations,SystemFolderExplorer};


#[derive(Debug)]
pub struct SharedCacheArch {
    pub shared_cache_name: String,
    pub symbols: CommonSymbolLocations
}

impl SharedCacheArch {
    fn new(shared_cache_name: String, symbols: CommonSymbolLocations) -> SharedCacheArch {
        SharedCacheArch{
            shared_cache_name,
            symbols
        }
    }
}


#[derive(Debug)]
pub struct SharedCacheLocation {
    pub location_name: String,
    pub shared_cachees: Vec<SharedCacheArch>
}

impl SharedCacheLocation {
    fn new(location_name: String) -> SharedCacheLocation {
        SharedCacheLocation{
            location_name,
            shared_cachees: Vec::new()
        }
    }

    fn push(&mut self, value: SharedCacheArch) {
        self.shared_cachees.push(value);
    }
}


#[derive(Debug)]
pub struct SharedCacheSymbolsLocation {
    locations: Vec<SharedCacheLocation>
}

impl SharedCacheSymbolsLocation {
    pub fn new(shared_cache_path: &Path, root_path: &Path, temp_path: &Path) -> SharedCacheSymbolsLocation {
        let mut shared_cache_system = SharedCacheSymbolsLocation {
            locations: Vec::new()
        };

        let mut explorer = SystemFolderExplorer::new(shared_cache_path);
        while !explorer.is_empty() {
            let temp_path = temp_path.join("dyld_shared_cache");
            if let Some((current_path,_)) = explorer.grab_folders() {
                let location_name = Self::get_shared_cache_location_name(&current_path,root_path);
                let temp_path = temp_path.join(location_name.as_str());

                let dyld_shared_cache_extractor = DyldSharedCacheExtractor::new(
                    &current_path, 
                    &temp_path
                );

                let mut shared_cache_location = SharedCacheLocation::new(location_name);
                for (shared_cache_name, extracted_path) in dyld_shared_cache_extractor.extracted_paths {
                    let common_symbols_location = CommonSymbolLocations::new(&extracted_path);

                    shared_cache_location.push(SharedCacheArch::new(shared_cache_name, common_symbols_location));
                }

                shared_cache_system.locations.push(shared_cache_location);
            }
        }

        shared_cache_system
    }


    pub fn get_shared_cache_list(&self) -> &Vec<SharedCacheLocation> {
        &self.locations
    }

    
    fn get_shared_cache_location_name(current_path: &Path, root_path: &Path) -> String {
        'calculate_location_name: {
            let Ok(current_path) = current_path.strip_prefix(root_path) else { break 'calculate_location_name };
            let Some(parent_name) = current_path.file_name() else { break 'calculate_location_name };
            let Some(sharedcache_location) = parent_name.to_str() else { break 'calculate_location_name };
            return sharedcache_location.to_string();
        };

        "RootFileSystem".to_string()
    }
}