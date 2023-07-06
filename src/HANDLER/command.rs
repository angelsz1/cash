use console::style;
use signal_hook::low_level::exit;
use std::{
    cmp::Ordering,
    env,
    fs::File,
    io::{Read, Write},
    path::Path,
    process::Command,
};

use crate::{os_info, parser};

#[allow(non_camel_case_types)]
enum SpecialCommands {
    cd,
    exit,
    None,
}

pub struct CommandStructure<'a> {
    pub cmd: String,
    pub args: Vec<&'a str>,
}

fn to_special_command(command: &str) -> SpecialCommands {
    if command.cmp("cd") == Ordering::Equal {
        return SpecialCommands::cd;
    }
    if command.cmp("exit") == Ordering::Equal {
        return SpecialCommands::exit;
    }
    return SpecialCommands::None;
}

fn build_command(parsed_commands: Vec<&str>) -> Result<CommandStructure, i16> {
    match to_special_command(parsed_commands[0]) {
        SpecialCommands::cd => {
            cd(parsed_commands);
            return Err(1);
        }
        SpecialCommands::exit => {
            set_last_dir(os_info::full_working_dir());
            exit(0);
        }
        SpecialCommands::None => {
            return Ok(CommandStructure {
                cmd: parsed_commands[0].to_string(),
                args: parsed_commands[1..].to_vec(),
            });
        }
    }
}

fn set_last_dir(path: String) {
    let file = File::create(os_info::current_history_path());
    match file {
        Ok(mut f) => {
            f.write_all(&path.as_bytes()).expect(
                format!(
                    "{}",
                    style("FATAL ERROR, couldn't write the log file.").red()
                )
                .as_str(),
            );
        }
        Err(_) => {
            let mut file = File::create(os_info::current_history_path()).expect(
                format!(
                    "{}",
                    style("FATAL ERROR, couldn't create the log file.").red()
                )
                .as_str(),
            );
            file.write_all(&path.as_bytes()).expect(
                format!(
                    "{}",
                    style("FATAL ERROR, couldn't write the log file.").red()
                )
                .as_str(),
            );
        }
    }
}

fn get_last_dir() -> String {
    let mut file = File::open(os_info::current_history_path())
        .expect(format!("{}", style("FATAL ERROR, log file doesn't exists.").red()).as_str());
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(format!("{}", style("FATAL ERROR, couldn't read log file.").red()).as_str());
    return contents;
}

fn cd(parsed_commands: Vec<&str>) {
    let mut root_str = os_info::current_homedir();
    if parsed_commands.len() != 1
        && parsed_commands[1].cmp("~") != Ordering::Equal
        && parsed_commands[1].cmp("-") != Ordering::Equal
    {
        root_str = parsed_commands[1].to_string();
    } else if parsed_commands.len() != 1 && parsed_commands[1].cmp("-") == Ordering::Equal {
        println!("{}{}", style("Going to : ").green(), get_last_dir());
        root_str = get_last_dir();
    }
    let root = Path::new(&root_str);
    let cur_dir = os_info::full_working_dir();
    let env_res = env::set_current_dir(&root);
    match env_res {
        Ok(_) => {
            set_last_dir(cur_dir);
        }
        Err(_) => {
            println!(
                "{}{}",
                style("Unknown directory: ").red(),
                parsed_commands[1]
            )
        }
    }
}

fn exec_command(parsed_commands: Vec<&str>) {
    let command = build_command(parsed_commands);
    match command {
        Err(_) => {}
        Ok(c) => {
            check_pipe_or_fifo(&c);
            let cmd = Command::new(&c.cmd).args(c.args).spawn();
            match cmd {
                Err(_) => {
                    println!(
                        "{}{}",
                        style("Unknown command: ").red(),
                        String::from(&c.cmd)
                    );
                }
                Ok(mut child) => {
                    child.wait().expect("AY MIGUEL");
                }
            }
        }
    }
}

//we could start by checking if pipe
fn check_pipe_or_fifo(command: &CommandStructure) {
    comman
}

pub fn exec_command_from_str(command: String) {
    let parsed_command = parser::parse_input(&command);
    self::exec_command(parsed_command.unwrap());
}
