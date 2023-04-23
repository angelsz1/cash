use std::env;
use users::{get_current_uid, get_user_by_uid};

pub fn current_working_dir() -> String {
    let pb = env::current_dir().unwrap();
    let as_str = pb.into_os_string().into_string().unwrap();
    let splits: Vec<&str> = as_str.split('/').collect();
    return format!("{}/{}", splits[splits.len() - 2], splits[splits.len() - 1]);
}

pub fn full_working_dir() -> String {
    let pb = env::current_dir().unwrap();
    let as_str = pb.into_os_string().into_string().unwrap();
    return as_str;
}

pub fn current_username() -> String {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    return String::from(user.name().to_string_lossy());
}

pub fn current_homedir() -> String {
    let mut home = "/home/".to_string();
    home.push_str(current_username().as_str());
    return home;
}

pub fn current_history_path() -> String {
    let mut hs = current_homedir();
    hs.push_str("/.cshistory");
    return hs;
}
