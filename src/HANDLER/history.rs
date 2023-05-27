use crate::os_info;
use crate::parser;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

static mut CONTENTS: Vec<String> = vec![];
static mut CUR_POS: usize = 0;

fn update_contents_and_pos() {
    let file_as_string = fs::read_to_string(get_filename()).unwrap();
    let ret_vec = parser::history_as_vec(file_as_string);
    unsafe {
        CUR_POS = ret_vec.len() - 1;
        CONTENTS = ret_vec;
    }
}

fn get_filename() -> String {
    let home = os_info::current_homedir();
    let mut filename = String::new();
    filename.push_str(&home);
    filename.push_str("/.cashistory");
    return filename;
}

pub fn set_up() {
    let filename = get_filename();
    let data = fs::read(format!("{}", filename));

    match data {
        Ok(_) => {}
        Err(_) => {
            fs::File::create(filename).expect("FATAL ERROR");
        }
    }
    update_contents_and_pos();
}

pub fn push_command(command: String) {
    let filename = get_filename();
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", command) {
        eprintln!("Couldn't write to file: {}", e);
    }
    update_contents_and_pos();
}

pub fn up() -> String {
    unsafe {
        if CUR_POS != 0 {
            CUR_POS -= 1;
        }
        return get_command();
    }
}

pub fn down() -> String {
    unsafe {
        if CUR_POS + 1 < CONTENTS.len() {
            CUR_POS += 1;
        }
        return get_command();
    }
}

fn get_command() -> String {
    unsafe {
        return String::from(CONTENTS.get(CUR_POS).unwrap());
    }
}
