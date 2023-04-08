use std::{cmp::Ordering, env, path::Path};

pub struct CommandStructure<'a> {
    pub cmd: String,
    pub args: Vec<&'a str>,
}

pub fn build_command(parsed_commands: Vec<&str>) -> Result<CommandStructure, i16> {
    if parsed_commands[0].cmp("cd") == Ordering::Equal {
        let mut root_str = "/home/angelsz";
        if parsed_commands.len() != 1 {
            root_str = parsed_commands[1];
        }
        let root = Path::new(root_str);
        env::set_current_dir(&root).expect("Couldn't change dir");
        return Err(1);
    }
    return Ok(CommandStructure {
        cmd: parsed_commands[0].to_string(),
        args: parsed_commands[1..].to_vec(),
    });
}
