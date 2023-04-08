pub fn parse_input(user_input: &String) -> Vec<&str> {
    return user_input.split(' ').collect(); //todo aceptar "" como un todo
}

// fn my_split(str: &String, ch: char) -> Vec<&str> {
//     let mut index = 0;
//     let mut last_index = 0;
//     let mut ret_vec: Vec<&str> = Vec::new();
//     let str_by = &mut str.chars();
//     let mut inside_double_quotes = false;
//     let mut inside_single_quotes = false;
//     while index < str_by.count() {
//         let nth = str.as_str().chars().nth(1).unwrap();
//         println!("{}", nth);

//         if nth == '"' {
//             inside_double_quotes = !inside_double_quotes;
//         } else if nth == '\'' {
//             inside_single_quotes = !inside_single_quotes;
//         } else if nth == ch && !inside_single_quotes && !inside_double_quotes {
//             ret_vec.push(&str.as_str()[last_index..index]);
//             println!("{}", &str.as_str()[last_index..index]);
//             last_index = index;
//         }

//         index = index + 1;
//     }
//     return ret_vec;
// }
