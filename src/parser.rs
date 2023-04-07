pub fn parse_input(user_input: &String) {
    let command_split: Vec<&str> = user_input.split(' ').collect();
    println!("{:?}", command_split);
}
