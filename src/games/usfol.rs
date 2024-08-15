use std::io::{stdin, stdout, Write};

pub fn user_input(print: &str) -> String{

    let mut user_input_string =  String::new();

    print!("{}", print);
    //--------USER INPUT--------\\

    // fluse input bufer
    let _=stdout().flush();

    // user input
    stdin().read_line(&mut user_input_string).expect("Did not enter a correct string");

    // remove newline carektors from input
    if let Some('\n')=user_input_string.chars().next_back() {
        user_input_string.pop();
    }
    if let Some('\r')=user_input_string.chars().next_back() {
        user_input_string.pop();
    }

    return user_input_string;
}

pub fn string_to_char(input: String) -> char{
    let char_vec: Vec<char> = input.chars().collect();
    return char_vec[0];
}