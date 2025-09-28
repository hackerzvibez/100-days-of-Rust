use std::io::{self, Write};
#[derive(Debug)]
enum ReverseError {
    Nullbyte,
    EmptyInput,
}

fn main() {
    until_user_end();
}

fn reverse_string(v: String) -> Result<String, ReverseError> {
    if v.is_empty() {
        return Err(ReverseError::EmptyInput);
    }

    if v.contains('\0') {
        return Err(ReverseError::Nullbyte);
    }
    let rev: String = v.chars().rev().collect();
    Ok(rev)
}

fn until_user_end() {
    loop {
        print!("Enter the String you want to reverse :");
        io::stdout().flush().expect("Failed to flush the output...");

        let mut str_: String = String::new();
        io::stdin()
            .read_line(&mut str_)
            .expect("Failed to read the input buffer");
        if str_.trim().is_empty() {
            break;
        }
        let original = str_.clone();
        let rev_str = reverse_string(str_);
        match rev_str {
            Ok(rev_str) => println!(
                "Original : {} \nReversed : {}",
                original.trim(),
                rev_str.trim()
            ),
            Err(ReverseError::EmptyInput) => println!("Error! empty input"),
            Err(ReverseError::Nullbyte) => println!("Error! nullbyte"),
        }
    }
}
