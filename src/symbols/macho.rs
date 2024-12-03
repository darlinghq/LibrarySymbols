use std::{path::Path, fs::File, io::Read};

const MACHO_32BIT_MH_MAGIC: u32 = 0xfeedface;
const MACHO_32BIT_MH_CIGAM: u32 = 0xcefaedfe;
const MACHO_64BIT_MH_MAGIC: u32 = 0xfeedfacf;
const MACHO_64BIT_MH_CIGAM: u32 = 0xcffaedfe;
const MACHO_FAT_MAGIC: u32 = 0xcafebabe;
const MACHO_FAT_CIGAM: u32 = 0xbebafeca;

pub fn is_file_macho<P: AsRef<Path>>(path: &P) -> bool {
    let path = path.as_ref();

    if !path.is_file() {
        return false
    }

    let macho_filetype: u32;
    if let Ok(mut file) = File::open(path) {
        let mut temp_buf: [u8; 4] = [0; 4];
        if let Ok(size) = file.read(&mut temp_buf) {
            if size != 4 { return false; }
        }
        macho_filetype = u32::from_le_bytes(temp_buf);
    } else {
        return false;
    }

    let macho_32bit: bool;
    let macho_64bit: bool;
    let macho_fat: bool;

    macho_32bit = macho_filetype == MACHO_32BIT_MH_MAGIC 
        || macho_filetype == MACHO_32BIT_MH_CIGAM;
    macho_64bit = macho_filetype == MACHO_64BIT_MH_MAGIC 
        || macho_filetype == MACHO_64BIT_MH_CIGAM;
    macho_fat = macho_filetype == MACHO_FAT_MAGIC 
        || macho_filetype == MACHO_FAT_CIGAM;

    macho_32bit || macho_64bit || macho_fat
}


pub fn is_file_proper_dyld_sharedcache(file_path: &Path) -> bool {
    
    let Some(file_name) = file_path.file_name() else { return false };
    let file_name = file_name.to_string_lossy();

    // There is proabaly a better way do deal with this, but I want to ignore
    // the following:
    //   -->  dyld_shared_cache_x86_64h.01
    //   -->  dyld_shared_cache_x86_64h.02
    //   -->  dyld_shared_cache_x86_64h.03
    //   -->  dyld_shared_cache_x86_64h.map
    // But this should NOT be ignored
    //   -->  dyld_shared_cache_x86_64h
    if file_path.is_file() && !file_name.contains(".") {
        if let Ok(mut file) = File::open(file_path) {
            let mut magic: [u8; 5] = [0; 5];
            if let Ok(_) = file.read(&mut magic) {
                // The magic header for sharedcache is "dyld_"
                return [b'd',b'y',b'l',b'd',b'_'] == magic;
            }
        }
    }

    return false
}
