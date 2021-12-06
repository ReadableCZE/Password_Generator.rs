use std;
use rand::{self, Rng};
use clipboard_win::{Clipboard, formats, Setter};

const CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMS: &str = "1234567890";
const SIMPOLS: &str = ",.-?:_\"!'/()=%\\#&@{}[]";



fn main() {
    println!("Welcome to password generator!");
    println!("This program will generate you random password with selected length and chars.");
    println!("After enterying length, new password will be automatically copied to clipboard.");
    println!("Lets get started!");
    println!("Enter options! You can select more option. <rc - random case, num - numbers, sim - simbols>");

    let mut string: String = String::new();

    // let mut _trash = std::io::stdout().flush();

    std::io::stdin().read_line(&mut string).expect("Error :(");

    let options: Vec<&str> = string.split_whitespace().collect();

    let mut password_list: String = String::new();
    password_list  += &CHARS.to_string();

    for option in options {
        
        if option == "rc" {
            password_list += &CHARS.to_uppercase();
        }
        if option == "num" {
            password_list  +=  &NUMS.to_string();

        }
        if option == "sim" {
            password_list  += &SIMPOLS.to_string();
        }
    }

    println!("And length:");
    
    string.clear();
    std::io::stdin().read_line(&mut string).expect("Error :(");

    match string.trim().parse() {
        Err(err) => println!("Provided number wasnt a number! Full error: {}", err),
        Ok(num) => {
            
            let mut password: String = String::new();
            let chars: Vec<char> = password_list.chars().collect();
            let chars_length: &usize  = &chars.len();
            for _i in 0..num {
                let random_number: usize = rand::thread_rng().gen_range(0..*chars_length);

                let choice: &char = &chars[random_number];

                password.push(*choice);

                drop(random_number);
                drop(choice);
            }

            println!("Your password has been copied to clipboard!");

            let _clip: Clipboard = Clipboard::new_attempts(10).expect("Open clipboard");
            formats::Unicode.write_clipboard(&password).expect("Write sample");

            println!("Password: {}", password);
        },
    }
}