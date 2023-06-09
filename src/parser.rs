pub fn parse_input(user_input: &String) -> Option<Vec<&str>> {
    return my_split(user_input);
}

fn my_split(str: &String) -> Option<Vec<&str>> {
    let mut last_index = 0;
    let mut ret_vec: Vec<&str> = Vec::new();
    let mut inside_double_quotes = false;
    let mut inside_single_quotes = false;

    for (i, c) in str.chars().enumerate() {
        if c == '"' {
            inside_double_quotes = !inside_double_quotes;
        } else if c == '\'' {
            inside_single_quotes = !inside_single_quotes;
        } else if c == ' ' && !inside_single_quotes && !inside_double_quotes {
            if str.chars().nth(last_index).unwrap() == '"' {
                ret_vec.push(&str.as_str()[last_index + 1..i - 1]);
            } else {
                ret_vec.push(&str.as_str()[last_index..i]);
            }
            last_index = i + 1;
        }
    }

    match str.chars().nth(last_index) {
        None => {
            return None;
        }
        Some(ch) => {
            if ch == '"' {
                ret_vec.push(&str.as_str()[last_index + 1..str.len() - 1]);
            } else {
                ret_vec.push(&str.as_str()[last_index..]);
            }
        }
    }
    return Some(ret_vec);
}

pub fn history_as_vec(contents: String) -> Vec<String> {
    return contents.split('\n').map(str::to_string).collect();
}
