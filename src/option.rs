use std::env;

#[derive(PartialEq)]
pub enum ArgsOption {
    FileIn,
    FileOut,
    FileDict,
    Other,
}

#[derive(Debug, Clone)]
pub struct ProgramOption {
    pub file_in: String,
    pub file_out: String,
    pub file_dict: String,
}

pub fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    return args[1..].to_vec();
}

pub fn parse_args(args: Vec<String>, min: usize) -> Option<ProgramOption> {
    if args.len() < min {
        return None;
    }

    let mut res = ProgramOption {
        file_dict: String::new(),
        file_out: String::new(),
        file_in: String::new(),
    };

    let mut counter = 0;
    while counter < args.len() {
        let mode = match args[counter].as_str() {
            "-i" | "--input" => ArgsOption::FileIn,
            "-o" | "--output" => ArgsOption::FileOut,
            "-d" | "--dictionary" => ArgsOption::FileDict,
            _ => ArgsOption::Other,
        };

        if let ArgsOption::Other = mode {
            counter += 1;
            continue;
        }

        counter += 1;
        if counter >= args.len() {
            return None;
        }

        let value = &args[counter];
        match mode {
            ArgsOption::FileIn => res.file_in = value.clone(),
            ArgsOption::FileOut => res.file_out = value.clone(),
            ArgsOption::FileDict => res.file_dict = value.clone(),
            ArgsOption::Other => {}
        }
        counter += 1;
    }
    Some(res)
}
