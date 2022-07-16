use std::{env::Args, path::{Path,PathBuf}};

pub struct Arguments {
    pub results_path: String,
    pub base_path: String
}

const RESULTS_DEFAULT_ARGUMENTS: usize = 0;
const MAX_DEFAULT_ARGUMENTS: usize = 1;

const EXPECT_MSG_NOT_ENOUGH_ARGUMENTS: &str = "Not enough arguments were provided";

struct OptionArugments {
    default_args: [Option<String>; MAX_DEFAULT_ARGUMENTS],
    default_args_cnt: usize,
    base_path: Option<String>
}

impl OptionArugments {
    fn new() -> OptionArugments {
        OptionArugments {
            default_args: [None; 1],
            default_args_cnt: 0,
            base_path: None 
        }
    }

    fn parse_commandline_arguments(&mut self, args: Args) {
        let args: Vec<String> = args.collect();

        let mut index = 1;
        while index < args.len() {
            let value = args.get(index).unwrap();
            
            let argument_type: Option<&str>;
            if value.starts_with("--") {
                argument_type = Some(&value[2..]);
            } else if value.starts_with("-") {
                argument_type = Some(&value[1..]);
            } else {
                argument_type = None;
                if self.default_args_cnt < MAX_DEFAULT_ARGUMENTS {
                    self.default_args[self.default_args_cnt] = Some(value.clone());
                    self.default_args_cnt +=1;
                }
            }

            if let Some(argument_type) = argument_type {
                index += 1;

                if "base_path" == argument_type {
                    let value = args.get(index).expect(EXPECT_MSG_NOT_ENOUGH_ARGUMENTS);
                    self.base_path = Some(value.clone());
                    println!("{}", value)
                } else {
                    panic!("\"{}\" argument is invalid", argument_type);
                }
            }

            index += 1;
        }
    }

    fn to_arguments(self) -> Arguments {
        let base_path = self.base_path.unwrap_or("/".to_string());

        Arguments {
            results_path: self.default_args[RESULTS_DEFAULT_ARGUMENTS].clone().expect(EXPECT_MSG_NOT_ENOUGH_ARGUMENTS),
            base_path
        }
    }
}

impl Arguments {
    pub fn new(args: Args) -> Arguments {
        let mut temp_arguments = OptionArugments::new();
        temp_arguments.parse_commandline_arguments(args);
        temp_arguments.to_arguments()
    }

    pub fn path_from_base(&self, location: &Path) -> PathBuf {
        let base_path = Path::new(self.base_path.as_str());
        base_path.join(location)
    }

    pub fn path_from_results(&self, location: &Path) -> PathBuf {
        let results_path = Path::new(self.results_path.as_str());
        results_path.join(location)
    }
}