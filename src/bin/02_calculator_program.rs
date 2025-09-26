use std::io::{self, Write};

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}
fn mul(a: i32, b: i32) -> i32 {
    return a * b;
}
fn div(a: i32, b: i32) -> i32 {
    return a / b;
}

fn fadd(a: f32, b: f32) -> f32 {
    return a + b;
}
fn fmul(a: f32, b: f32) -> f32 {
    return a * b;
}
fn fdiv(a: f32, b: f32) -> f32 {
    return a / b;
}
fn fsub(a: f32, b: f32) -> f32 {
    return a - b;
}
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

    if is_int(a) && is_int(b) {
        let i_a: i32 = a.parse().unwrap();
        let i_b: i32 = b.parse().unwrap();
        let result = match op {
            '+' => add(i_a, i_b),
            '-' => sub(i_a, i_b),
            '*' => mul(i_a, i_b),
            '/' => {
                if i_b == 0 {
                    println!("<Error!> division by zero");
                    return;
                }
                div(i_a, i_b)
            }
            _ => {
                println!("<Error!> unknown operator!");
                return;
            }
        };
        println!("Result : {} ", result)
    } else {
        let f_a: f32 = a.parse().unwrap();
        let f_b: f32 = b.parse().unwrap();
        let result = match op {
            '+' => fadd(f_a, f_b),
            '-' => fsub(f_a, f_b),
            '*' => fmul(f_a, f_b),
            '/' => {
                if f_b == 0.0 {
                    println!("<Error!> division by zero");
                    return;
                }
                fdiv(f_a, f_b)
            }
            _ => {
                println!("<Error!> unknown operator!");
                return;
            }
        };
        println!("Result : {}", result);
    }
}
