use std::{process::{Command, Output}};

fn parse_stdout(output: Output) -> Vec<String> {
    let raw_string = String::from_utf8(output.stdout).expect("Unable to save output");
    
    let mut vec: Vec<String> = Vec::new();
    for line in raw_string.lines() {
        vec.push(line.to_string());
    }

    vec
}

const PRODUCT_BUILD_VERSION: &str = "ProductBuildVersion";
const PRODUCT_NAME: &str = "ProductName";
const PRODUCT_VERSION: &str = "ProductVersion";
#[derive(Debug)]
pub struct SystemVersionDefaults {
    pub product_build_version: String,
    pub product_name: String,
    pub product_version: String
}

impl SystemVersionDefaults {
    pub fn new(path: &str) -> SystemVersionDefaults {
        let product_build_version = SystemVersionDefaults::launch_program(path, PRODUCT_BUILD_VERSION);
        let product_name = SystemVersionDefaults::launch_program(path, PRODUCT_NAME);
        let product_version = SystemVersionDefaults::launch_program(path, PRODUCT_VERSION);

        SystemVersionDefaults {
            product_build_version,
            product_name,
            product_version
        }
    }

    fn launch_program(path: &str, key: &str) -> String {
        let output = Command::new("defaults")
            .args(["read", path, key])
            .output()
            .expect("Unable to launch 'defaults' application");

        parse_stdout(output).first().expect("Unable to obtain value").to_string()
    }
}