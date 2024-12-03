use std::{ffi::{c_char, CString}, fs::{read_dir, ReadDir}, path::{Path, PathBuf}, ptr};

use crate::symbols::macho::is_file_proper_dyld_sharedcache;

extern "C" {
    fn launch_dyld_shared_cache_extractor_program(shared_cache_cstr: *const c_char, library_candidate_cstr: *const c_char, output_path_cstr: *const c_char) -> i32;
}

#[derive(Debug)]
pub struct DyldSharedCacheExtractor {
    pub extracted_paths: Vec<(String,PathBuf)>
}

impl DyldSharedCacheExtractor {
    pub fn new(root_path: &Path, extraction_path: &Path) -> DyldSharedCacheExtractor {
        let mut instance =  DyldSharedCacheExtractor {
            extracted_paths: Vec::new()
        };


        let sharedcache_path = Self::get_shared_cache_folder_path(root_path);
        println!("Inspecting {} directory for shared cache", sharedcache_path.to_string_lossy());
        if let Ok(sharedcache_readdir) = read_dir(sharedcache_path) {
            instance.extract_shared_library(sharedcache_readdir, extraction_path);
        }

        instance
    }

    fn get_shared_cache_folder_path(path: &Path) -> PathBuf {
        // TODO: Support non-macOS shared cache location
        // iPhoneOS: "Library/Caches/com.apple.dyld"

        // macOS
        path.join("System/Library/dyld")
    }


    fn extract_shared_library(&mut self, sharedcache_readdir: ReadDir, extracted_path: &Path) {
        for sharedcache_direntry in sharedcache_readdir {
            let Ok(sharedcache_direntry) = sharedcache_direntry else { continue };
            let sharedcache_file_path = sharedcache_direntry.path();

            if is_file_proper_dyld_sharedcache(sharedcache_file_path.as_path()) {
                let Some(file_name) = sharedcache_file_path.file_name() else { continue };
                let Some(file_name) = file_name.to_str() else { continue };
                let temp_path = extracted_path.join(&format!("{}.dir",file_name));

                // If the path doesn't exist after `dyld-shared-cache-extractor` finishes executing, it means that
                // the application was not able to extract anything from it.
                DyldSharedCacheExtractor::launch_program(sharedcache_file_path.as_path(), &temp_path);
                if temp_path.is_dir() {
                    self.extracted_paths.push( (file_name.to_string(),temp_path) );
                }
            }
        }
    }


    fn launch_program(shared_cache_path: &Path, temp_path: &Path) {
        unsafe {
            let Some(shared_cache_path) = shared_cache_path.to_str() else { return };
            let Some(temp_path) = temp_path.to_str() else { return };

            let Ok(shared_cache_cstr) = CString::new(shared_cache_path) else { return };
            let Ok(output_path_cstr) = CString::new(temp_path) else { return };

            launch_dyld_shared_cache_extractor_program(
                shared_cache_cstr.as_ptr(), 
                ptr::null(), 
                output_path_cstr.as_ptr());
        }


        // let _ = Command::new("dyld-shared-cache-extractor")
        //     .args([shared_cache_path, temp_path])
        //     .status()
        //     .expect("Unable to launch 'dyld-shared-cache-extractor' application");
    }
}
