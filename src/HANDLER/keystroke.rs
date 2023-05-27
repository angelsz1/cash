use crate::command;
use crate::history;
use crate::infobar;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn handle_strokes() -> Option<String> {
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    let mut buffer = String::new();
    let bar_len = infobar::get_bar_length();
    let mut cursor_pos = bar_len;

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
                        history::push_command(String::from(&buffer));
                        return Some(buffer);
                    } else {
                        return None;
                    }
                }
                buffer.insert(cursor_pos - bar_len, c);
                cursor_pos += 1;
                refresh(bar_len, &buffer, cursor_pos, None);
            }
            Key::Home => {
                print!(
                    "{}",
                    termion::cursor::Left((cursor_pos - bar_len).try_into().unwrap())
                );
                cursor_pos = bar_len;
            }
            Key::Backspace => {
                if cursor_pos > bar_len {
                    buffer.remove(cursor_pos - bar_len - 1);
                    cursor_pos -= 1;
                    refresh(bar_len, &buffer, cursor_pos, None);
                }
            }
            Key::Left => {
                if cursor_pos > bar_len {
                    print!("{}", termion::cursor::Left(1));
                    cursor_pos -= 1;
                }
            }
            Key::Right => {
                if cursor_pos < bar_len + buffer.len() {
                    print!("{}", termion::cursor::Right(1));
                    cursor_pos += 1;
                }
            }
            Key::Up => {
                buffer = history::up();
                refresh(bar_len, &buffer, cursor_pos, Some(bar_len + buffer.len()));
                cursor_pos = bar_len + buffer.len();
            }
            Key::Down => {
                buffer = history::down();
                refresh(bar_len, &buffer, cursor_pos, Some(bar_len + buffer.len()));
                cursor_pos = bar_len + buffer.len();
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
    drop(stdout);
    history::push_command(String::from(&buffer));
    return Some(buffer);
}

fn refresh(bar_len: usize, buffer: &String, cursor_pos: usize, new_cursor_pos: Option<usize>) {
    print!(
        "{}{}",
        termion::clear::CurrentLine,
        termion::cursor::Left((bar_len + buffer.len() + 1).try_into().unwrap())
    );
    infobar::show_infobar();
    match new_cursor_pos {
        Some(num) => {
            print!(
                "{}{}{}",
                buffer,
                termion::cursor::Left((bar_len + buffer.len() - num + 1).try_into().unwrap()),
                termion::cursor::Right(1)
            );
        }
        None => {
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
    }
}

fn ctrl_c() {
    println!();
}
