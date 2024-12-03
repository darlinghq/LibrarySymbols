use std::{path::Path, process::Command};

use super::parse_stdout;

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
    pub fn new(path: &Path) -> SystemVersionDefaults {
        let system_version_path = path.join(Path::new("System/Library/CoreServices/SystemVersion.plist"));
        assert!(system_version_path.is_file(), "Unable to locate SystemVersion.plist");

        let product_build_version = SystemVersionDefaults::launch_program(&system_version_path, PRODUCT_BUILD_VERSION);
        let product_name = SystemVersionDefaults::launch_program(&system_version_path, PRODUCT_NAME);
        let product_version = SystemVersionDefaults::launch_program(&system_version_path, PRODUCT_VERSION);

        SystemVersionDefaults {
            product_build_version,
            product_name,
            product_version
        }
    }

    fn launch_program(path: &Path, key: &str) -> String {
        let output = Command::new("defaults")
            .args(["read", path.to_str().expect("Unable to convert path to string"), key])
            .output()
            .expect("Unable to launch 'defaults' application");

        parse_stdout(output).first().expect("Unable to obtain value").to_string()
    }
}