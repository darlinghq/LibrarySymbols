use std::process::Command;

use super::parse_stdout;

#[derive(Debug)]
pub struct WhoAmIUserName {
    pub macos_users_dir: String
}

impl WhoAmIUserName {
    pub fn new() -> WhoAmIUserName {
        let username = WhoAmIUserName::launch_program();
        let macos_users_dir = format!("/Users/{}",username);

        WhoAmIUserName {
            macos_users_dir
        }
    }

    fn launch_program() -> String {
        let output = Command::new("whoami")
        .output()
        .expect("Unable to launch 'whoami' application");

        parse_stdout(output).first().expect("Unable to obtain value").to_string()
    }

    pub fn mask_user_account(&self, value: &String) -> String {
        value.replace(self.macos_users_dir.as_str(), "/Users/[Removed Username]")
    }
}