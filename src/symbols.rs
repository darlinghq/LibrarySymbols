use std::{path::{PathBuf, Path}, fs::{File, self}, io::{Read, Write}};

use crate::{location, program};

const SYSTEM_LIBRARY_FRAMEWORK_PATH: &str = "System/Library/Frameworks";
const SYSTEM_LIBRARY_PRIVATEFRAMEWORK_PATH: &str = "System/Library/PrivateFrameworks";
const USR_LIB_PATH: &str = "usr/lib";

const MACHO_32BIT_MH_MAGIC: u32 = 0xfeedface;
const MACHO_32BIT_MH_CIGAM: u32 = 0xcefaedfe;
const MACHO_64BIT_MH_MAGIC: u32 = 0xfeedfacf;
const MACHO_64BIT_MH_CIGAM: u32 = 0xcffaedfe;
const MACHO_FAT_MAGIC: u32 = 0xcafebabe;
const MACHO_FAT_CIGAM: u32 = 0xbebafeca;

fn is_file_macho(path: &PathBuf) -> bool {
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

pub struct ParseBaseFilesystem {
    framework_path: PathBuf,
    privateframework_path: PathBuf,
    usr_lib_path: PathBuf
}

const NM_TEXTFILE_NAME: &str = "nm.txt";
const OTOOL_TEXTFILE_NAME: &str = "otool.txt";

impl ParseBaseFilesystem {
    pub fn new<P: AsRef<Path>>(location: P) -> ParseBaseFilesystem {
        let framework_path = location.as_ref().join(SYSTEM_LIBRARY_FRAMEWORK_PATH);
        let privateframework_path = location.as_ref().join(SYSTEM_LIBRARY_PRIVATEFRAMEWORK_PATH);
        let usr_lib_path = location.as_ref().join(USR_LIB_PATH);
        
        ParseBaseFilesystem {
            framework_path,
            privateframework_path,
            usr_lib_path
        }
    }

    pub fn traverse<P: AsRef<Path>, Q: AsRef<Path>>(&self, result_location: P, unique_folder: Option<&str>, base_location: Q, whoami: &program::WhoAmIUserName) {
        let mut macho_paths: Vec<PathBuf> = Vec::new();
        macho_paths.append(&mut location::walk_directory(&self.framework_path, is_file_macho));
        macho_paths.append(&mut location::walk_directory(&self.privateframework_path, is_file_macho));
        macho_paths.append(&mut location::walk_directory(&self.usr_lib_path, is_file_macho));

        let macho_paths = macho_paths;
        for macho_path in macho_paths.iter() {
            let relative_location = macho_path.strip_prefix(base_location.as_ref()).unwrap();
            let result_dir: PathBuf = if let Some(unique_folder) = unique_folder {
                result_location.as_ref().join(unique_folder).join(relative_location)
            } else {
                result_location.as_ref().join(relative_location)
            };

            println!("Parsing {:?}", macho_path);
            fs::create_dir_all(&result_dir).expect("Unable to create directory");

            let mut nm_textfile = File::create(result_dir.as_path().join(NM_TEXTFILE_NAME)).expect("Unable to create file");
            let nm = program::NmLibrarySymbols::new(&macho_path);
            nm_textfile.write(nm.raw_output.as_bytes()).expect("Unable to save log information into file");

            let mut otool_textfile = File::create(result_dir.as_path().join(OTOOL_TEXTFILE_NAME)).expect("Unable to create file");
            let otool = program::OtoolLibrarySymbols::new(&macho_path,whoami);
            otool_textfile.write(otool.raw_output.as_bytes()).expect("Unable to save log information into file");

        }
    }
}