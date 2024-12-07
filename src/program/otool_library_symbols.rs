use std::{path::Path, process::Command};

use super::{SaveOutput, WhoAmIUserName};

#[derive(Debug)]
pub struct OtoolLibrarySymbols {
    pub raw_output: String
}

impl OtoolLibrarySymbols {
    pub fn new(macho: &Path, whoami: &WhoAmIUserName) -> OtoolLibrarySymbols {
        let raw_output = OtoolLibrarySymbols::launch_program(macho,whoami);

        OtoolLibrarySymbols {
            raw_output
        }
    }

    fn launch_program(macho: &Path, whoami: &WhoAmIUserName) -> String {
        let Some(macho_str) = macho.to_str() else { return String::new() };

        let output = Command::new("otool")
        .args(["-L", macho_str, "-arch", "all"])
        .output()
        .expect("Unable to launch 'otool' application");

        let output = String::from_utf8(output.stdout).expect("Unable to save output");
        whoami.mask_user_account(&output)
    }
}

impl SaveOutput for OtoolLibrarySymbols {
    fn get_raw_output(&self) -> &String {
        &self.raw_output
    }
}
