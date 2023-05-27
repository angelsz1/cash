use crate::command;
use crate::infobar;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

//TODO change arrow movement to function like add and delete
pub fn handle_strokes() -> Option<String> {
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    let mut buffer = String::new();
    let bar_len = infobar::get_bar_length();
    let mut cursor_pos = bar_len;
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
            Key::Ctrl('w') => {
                if buffer.len() > 0 {
                    let original_len = buffer.len();
                    let buf_splitted = buffer.split_whitespace();
                    let buf_splitted2 = buffer.split_whitespace(); //feo
                    let take_except_last = buf_splitted.take(buf_splitted2.count() - 1);
                    let chars: Vec<&str> = take_except_last.collect();
                    let result: String = chars.join(" ");
                    buffer = result;
                    print!(
                        "{}{}",
                        termion::clear::CurrentLine,
                        termion::cursor::Left((bar_len + original_len).try_into().unwrap())
                    );
                    infobar::show_infobar();
                    print!(
                        "{}{}{}",
                        buffer,
                        termion::cursor::Left(
                            (bar_len + original_len - cursor_pos).try_into().unwrap()
                        ),
                        termion::cursor::Right(1)
                    );
                    cursor_pos -= original_len - buffer.len();
                }
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
                print!(
                    "{}{}",
                    termion::clear::CurrentLine,
                    termion::cursor::Left((bar_len + buffer.len() + 1).try_into().unwrap())
                );
                infobar::show_infobar();
                buffer.insert(cursor_pos - initial_cursor_pos, c);
                cursor_pos += 1;
                print!(
                    "{}{}{}",
                    buffer,
                    termion::cursor::Left(
                        (bar_len + buffer.len() - cursor_pos + 1)
                            .try_into()
                            .unwrap()
                    ),
                    termion::cursor::Right(1)
                );
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
                    buffer.remove(cursor_pos - initial_cursor_pos - 1);
                    cursor_pos -= 1;
                    print!(
                        "{}{}",
                        termion::clear::CurrentLine,
                        termion::cursor::Left((bar_len + buffer.len() + 1).try_into().unwrap())
                    );
                    infobar::show_infobar();
                    print!(
                        "{}{}{}",
                        buffer,
                        termion::cursor::Left(
                            (bar_len + buffer.len() - cursor_pos).try_into().unwrap()
                        ),
                        termion::cursor::Right(1)
                    );
                }
            }
            Key::Left => {
                if cursor_pos > initial_cursor_pos {
                    print!("{}", termion::cursor::Left(1));
                    cursor_pos -= 1;
                }
            }
            Key::Right => {
                if cursor_pos < initial_cursor_pos + buffer.len() {
                    print!("{}", termion::cursor::Right(1));
                    cursor_pos += 1;
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
