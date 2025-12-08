use std::fmt::Display;

pub mod common;

pub fn run_part<T: Display, F, I>(function: F, input: I, part: u8)
where
    F: Fn(I) -> Option<T>,
    I: Copy,
{
    if let Some(res) = function(input) {
        println!("part {part}: {res}")
    } else {
        println!("part {part} failed")
    }
}
