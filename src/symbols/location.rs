use crate::arguments::library_symbols::LibrarySymbolsArguments;
use crate::symbols::location::common::CommonSymbolLocations;
use crate::symbols::location::shared_cache::SharedCacheSymbolsLocation;

pub mod common;
pub mod macho_executables;
pub mod shared_cache;

pub trait SymbolsGetter {
    fn get_root_symbols(&self) -> &CommonSymbolLocations;
    fn get_shared_cache_symbols(&self) -> &SharedCacheSymbolsLocation;
}


#[derive(Debug)]
pub struct RootFileSystemLocation {
    root_symbols: CommonSymbolLocations,
    shared_cache_symbols: SharedCacheSymbolsLocation,
}

impl RootFileSystemLocation {
    pub fn new(library_symbols: &LibrarySymbolsArguments) -> RootFileSystemLocation {
        
        let root_symbols = CommonSymbolLocations::new(library_symbols.get_root_path());
        let shared_cache_symbols = SharedCacheSymbolsLocation::new(
            library_symbols.get_root_path(),
            library_symbols.get_root_path(),
            &library_symbols.get_temp_path().join("sharedcache")
        );

        RootFileSystemLocation {
            root_symbols,
            shared_cache_symbols
        }
    }
}

impl SymbolsGetter for RootFileSystemLocation {
    fn get_root_symbols(&self) -> &CommonSymbolLocations {
        &self.root_symbols
    }

    fn get_shared_cache_symbols(&self) -> &SharedCacheSymbolsLocation {
        &self.shared_cache_symbols
    }
}


#[derive(Debug)]
pub struct CryptexesOsLocation {
    root_symbols: CommonSymbolLocations,
    shared_cache_symbols: SharedCacheSymbolsLocation,
}

impl CryptexesOsLocation {
    pub fn new(library_symbols: &LibrarySymbolsArguments) -> Option<CryptexesOsLocation> {

        if let Some(cryptexes_os_path) = library_symbols.get_cryptexes_os_path() {
            let root_symbols = CommonSymbolLocations::new(cryptexes_os_path);
            let shared_cache_symbols = SharedCacheSymbolsLocation::new(
                cryptexes_os_path,
                cryptexes_os_path,
                &library_symbols.get_temp_path().join("cryptexes_os")
            );
    
            return Some(CryptexesOsLocation {
                root_symbols,
                shared_cache_symbols
            });
        }
        
        return None;
    }
}

impl SymbolsGetter for CryptexesOsLocation {
    fn get_root_symbols(&self) -> &CommonSymbolLocations {
        &self.root_symbols
    }

    fn get_shared_cache_symbols(&self) -> &SharedCacheSymbolsLocation {
        &self.shared_cache_symbols
    }
}
