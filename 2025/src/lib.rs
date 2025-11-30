use std::fmt::Display;

pub mod common;

pub fn run_part<'a, T: Display, F>(function: F, input: &'a str, part: u8)
where
    F: Fn(&'a str) -> Option<T>,
{
    if let Some(res) = function(input) {
        println!("part {part}: {res}")
    } else {
        println!("part {part} failed")
    }
}
