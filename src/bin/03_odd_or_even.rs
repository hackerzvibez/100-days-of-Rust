use std::io::{self, Write};

fn main() {
    odd_or_even(5);
    odd_or_even_user_in();
}

fn odd_or_even(v: i64) {
    if v % 2 == 0 {
        println!("<EVEN> {} is an even number!", v);
    } else {
        println!("<ODD> {} is an odd number!", v);
    }
}
fn odd_or_even_user_in() {
    let mut v: String = String::new();
    print!("Enter the number you want to find whether it is <odd> or <even> :");
    io::stdout()
        .flush()
        .expect("Failed to flush the output buffer");

    io::stdin()
        .read_line(&mut v)
        .expect("Failed the read the input buffer");

    match v.trim().parse::<i64>() {
        Ok(n) => odd_or_even(n),
        Err(_) => {
            println!("<Error!> invalid input");
            return;
        }
    };
}
