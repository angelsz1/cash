use crate::command;
use crate::infobar;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn handle_strokes() -> Option<String> {
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    let mut buffer = String::new();
    let mut cursor_pos = infobar::get_bar_length();
    let initial_cursor_pos = cursor_pos;

    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => {
                drop(stdout);
                ctrl_c();
                return None;
            }
            Key::Ctrl('l') => {
                command::exec_command_from_str("clear".to_string());
                infobar::show_infobar();
                print!("{}", buffer);
            }
            Key::Char(c) => {
                if c == '\n' {
                    drop(stdout);
                    println!();
                    if buffer.len() > 0 {
                        return Some(buffer);
                    } else {
                        return None;
                    }
                }
                print!("{}", c);
                buffer.insert(cursor_pos - initial_cursor_pos, c);
                cursor_pos += 1;
            }
            Key::Home => {
                print!(
                    "{}",
                    termion::cursor::Left((cursor_pos - initial_cursor_pos).try_into().unwrap())
                );
                cursor_pos = initial_cursor_pos;
            }
            Key::Backspace => {
                if cursor_pos > initial_cursor_pos {
                    print!(
                        "{} {}{}",
                        termion::cursor::Left(1),
                        termion::clear::AfterCursor,
                        termion::cursor::Left(1)
                    );
                    buffer.remove(cursor_pos - initial_cursor_pos - 1);
                    cursor_pos -= 1;
                }
            }
            Key::Left => {
                if cursor_pos > initial_cursor_pos {
                    print!("{}", termion::cursor::Left(1));
                    cursor_pos -= 1;
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
    drop(stdout);
    return Some(buffer);
}

fn ctrl_c() {
    println!();
}
