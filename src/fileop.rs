use crate::option::ProgramOption;
use std::collections::HashMap;
use std::fs;
use toml;

pub fn parse_dict_file(po: &ProgramOption) -> Option<HashMap<String, String>> {
    let dict_file = read_file(&po.file_dict);
    if dict_file.is_err() {
        return None;
    }
    let dict_res = parse_dict_from_str(&dict_file.unwrap());
    return Some(dict_res.unwrap());
}

pub fn replace_in(file_path_in: &str, dict: &HashMap<String, String>, delimiter: &str) -> String {
    let in_str = read_file(file_path_in);
    if in_str.is_err() {
        return "".to_string();
    }
    let mut res: String = in_str.clone().unwrap();
    for (key, val) in dict.into_iter() {
        let build_str: &str = &std::format!("{}{}{}", delimiter, key, delimiter);
        res = res.replace(build_str, val);
    }
    return res;
}

pub fn write_to(file_path_out: &str, data: &str) -> bool {
    let _ = fs::write(file_path_out, data.as_bytes()).map_err(|err| {
        eprintln!("ERROR: Failed to write {} because : {}", file_path_out, err);
        return false;
    });
    return true;
}

fn read_file(fpath: &str) -> Result<String, ()> {
    let fstr = fs::read_to_string(fpath).map_err(|err| {
        eprintln!("ERROR: Failed to read the file `{}` : {}", fpath, err);
        return Err::<String, ()>(());
    });
    return Ok(fstr.unwrap());
}

fn parse_dict_from_str(dict_file: &str) -> Result<HashMap<String, String>, ()> {
    let dict_hashmap: Result<HashMap<String, String>, ()> =
        toml::from_str(dict_file).map_err(|e| {
            eprintln!("ERROR: Failed to parse the string : {}", e);
            return ();
        });
    if dict_hashmap.is_err() {
        return Err(());
    }
    return dict_hashmap;
}
