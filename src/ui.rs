use std::io::{stdout, Write};
use color_print::{cformat, cprintln};

pub fn print_status<T>(input: T)
    where
        T: std::fmt::Display,
{
    cprintln!("<#0DB2F8><bold>[*]</></> {}", input);
}

pub fn print_error<T>(input: T)
    where
        T: std::fmt::Display,
{
    cprintln!("<#fca503><bold>[!]</></> {}", input);
}

pub fn print_success<T>(input: T)
    where
        T: std::fmt::Display,
{
    cprintln!("<#3cb521><bold>[+]</></> {}", input);
}
pub fn bold<T>(input: T) -> String
    where
        T: std::fmt::Display,
{
    cformat!("<bold>{}</bold>", input)
}

pub fn readline<T>(prompt: T) -> String
    where
        T: std::fmt::Display
{
    print!("{}", prompt);
    stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}