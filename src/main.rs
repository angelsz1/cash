use std::io::Write;
#[path = "../src/HANDLER/command.rs"]
mod command;
mod infobar;
#[path = "../src/HANDLER/keystroke.rs"]
mod keystroke;
mod os_info;
mod parser;

fn main() {
    loop {
        infobar::show_infobar();
        std::io::stdout().flush().expect("flush failed!");
        let stroke_option = keystroke::handle_strokes();
        match stroke_option {
            None => {}
            Some(pi) => {
                command::exec_command_from_str(pi);
            }
        }
    }
}
