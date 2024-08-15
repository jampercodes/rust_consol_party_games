

pub fn user_input(Print: &str) -> String{
    
    let mut user_input_string =  String::new();

    print!("{}", Print);
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