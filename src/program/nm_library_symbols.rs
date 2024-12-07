use std::{path::Path, process::Command};

use crate::program::SaveOutput;
use crate::program::WhoAmIUserName;

#[derive(Debug)]
pub struct NmLibrarySymbols {
    pub raw_output: String
}

impl NmLibrarySymbols {
    pub fn new(macho: &Path, whoami: &WhoAmIUserName) -> NmLibrarySymbols {
        let raw_output = NmLibrarySymbols::launch_program(macho,whoami);
        
        NmLibrarySymbols {
            raw_output
        }
    }

    fn launch_program(macho: &Path, whoami: &WhoAmIUserName) -> String {
        let Some(macho_str) = macho.to_str() else { return String::new() };

        let output = Command::new("nm")
        .args(["-m", macho_str, "-arch", "all"])
        .output()
        .expect("Unable to launch 'nm' application");

        let output = String::from_utf8(output.stdout).expect("Unable to save output");
        whoami.mask_user_account(&output)
    }
}

impl SaveOutput for NmLibrarySymbols {
    fn get_raw_output(&self) -> &String {
        &self.raw_output
    }
}
