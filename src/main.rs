mod fileop;
mod option;
use fileop::*;
use option::*;

enum ExitType {
    Success,
    NotEnoughtArgs,
    BrokenDict,
    FailedToWrite,
}

fn exit_with_code(code: ExitType) {
    std::process::exit(code as i32);
}

fn main() {
    let prog_args: Vec<String> = get_args();
    let res: Option<ProgramOption> = parse_args(prog_args, 3 * 2);
    if res.is_some() {
        let dict = parse_dict_file(&res.clone().unwrap());
        if dict.is_none() {
            exit_with_code(ExitType::BrokenDict);
        }
        let result = replace_in(&res.clone().unwrap().file_in, &dict.unwrap());
        if !write_to(&res.unwrap().file_out, &result) {
            exit_with_code(ExitType::FailedToWrite);
        }
        exit_with_code(ExitType::Success);
    }
    exit_with_code(ExitType::NotEnoughtArgs);
}
