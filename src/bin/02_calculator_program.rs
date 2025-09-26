use std::io::{self, Write};

fn main() {
    //getting operator from the user
    let mut c: String = String::new();
    print!("Enter the operation you want to perform(+,-,*,/) :");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut c).unwrap();
    let op = c.trim().chars().next().unwrap();

    //getting the number a from the user
    let mut a: String = String::new();
    print!("Enter The number A :");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a).unwrap();
    let a = a.trim();

    //getting the number b from the user
    let mut b: String = String::new();
    print!("Enter the Number B :");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b).unwrap();
    let b = b.trim();

    let is_int = |s: &str| !s.contains('.');

    if is_int(a) && is_int(b) {}
}
