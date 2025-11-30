use std::{fs, path::PathBuf};

use crate::cli::{YEAR, util::save_to_file};

const TEMPLATE_PATH: &str = "./template.txt";
const DAY_DIR: &str = "./src/bin";

fn get_content(day: u8) -> Result<String, String> {
    let filepath = PathBuf::from(TEMPLATE_PATH);
    let template = fs::read_to_string(filepath).map_err(|e| e.to_string())?;
    Ok(template.replace("{day}", &day.to_string()))
}

fn save_new_day(content: String, day: u8) -> Result<(), String> {
    let mut filepath = PathBuf::new();
    let filename = format!("day{:0>2}-{}.rs", day, YEAR);
    filepath.push(DAY_DIR);
    filepath.push(filename);
    if filepath.exists() {
        println!(
            "the file {} already exists. skipping scaffolding.",
            filepath.to_str().unwrap_or("<INVALID_PATH>"),
        );
        return Ok(());
    }

    save_to_file(content, &filepath)
}

pub fn scaffold(day: u8) -> Result<(), String> {
    println!("Scaffolding day {}...", day);

    let content = get_content(day)?;
    save_new_day(content, day)?;

    Ok(())
}
