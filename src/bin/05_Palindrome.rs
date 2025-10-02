use std::io::{self, Write};
fn main() {
    until_user_end();
}

fn palindrome(v: &String) -> Result<bool, ()> {
    let chars: Vec<char> = v.chars().collect();
    let mut low: usize = 0;
    let mut high: usize = chars.len().saturating_sub(1);
    while low < high {
        if chars[low] != chars[high] {
            return Ok(false);
        }
        low += 1;
        high = high.saturating_sub(1);
    }
    Ok(true)
}

fn until_user_end() {
    println!("===========PalindromeChecker============");
    loop {
        print!("Enter the Word :");
        io::stdout().flush().expect("<Error!> failed to flush");
        let mut string: String = String::new();
        io::stdin()
            .read_line(&mut string)
            .expect("<Error!> Failed to read buffer");
        string = string.trim().to_string();
        if string.is_empty() {
            println!("Exiting...");
            return;
        }
        match palindrome(&string) {
            Ok(true) => println!("It is a palindrome"),
            Ok(false) => println!("It is not a palindrome"),
            Err(()) => println!("<Error!>While Checking for palindrome"),
        }
    }
}
