use std::{path::Path, process::Command};

use crate::program::SaveOutput;

#[derive(Debug)]
pub struct NmLibrarySymbols {
    pub raw_output: String
}

impl NmLibrarySymbols {
    pub fn new(macho: &Path) -> NmLibrarySymbols {
        let raw_output = NmLibrarySymbols::launch_program(macho);
        
        NmLibrarySymbols {
            raw_output
        }
    }

    fn launch_program(macho: &Path) -> String {
        let Some(macho_str) = macho.to_str() else { return String::new() };

        let output = Command::new("nm")
        .args(["-m", macho_str, "-arch", "all"])
        .output()
        .expect("Unable to launch 'nm' application");

        String::from_utf8(output.stdout).expect("Unable to save output")
    }
}

impl SaveOutput for NmLibrarySymbols {
    fn get_raw_output(&self) -> &String {
        &self.raw_output
    }
}
