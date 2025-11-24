use core::fmt;
use dotenv::dotenv;
use std::{env, error::Error, path::PathBuf};

use crate::cli::util::save_to_file;

const YEAR: u32 = 2024; // set to 2024 for debugging; 2025 will be available in december
const INPUT_DIR: &str = "./data/input";
const DESC_DIR: &str = "./data/description";

#[derive(Debug)]
struct DownloadError(String);

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Download Error: {}", self.0)
    }
}

impl Error for DownloadError {}

fn get_response(url: String, session_token: &str) -> Result<String, DownloadError> {
    let cookie = format!("session={}", session_token);

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::COOKIE, cookie)
        .send()
        .map_err(|e| DownloadError(e.to_string()))?
        .text()
        .map_err(|e| DownloadError(e.to_string()))?;

    Ok(res)
}

fn download_input(day: u8, session_token: &str) -> Result<(), String> {
    let mut filepath = PathBuf::new();
    let filename = format!("day{:0>2}-{}.txt", day, YEAR);
    filepath.push(INPUT_DIR);
    filepath.push(filename);
    if filepath.exists() {
        println!(
            "the file {} already exists. skipping download.",
            filepath.to_str().unwrap_or("<INVALID_PATH>"),
        );
        return Ok(());
    }

    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);
    let res = get_response(url, session_token).map_err(|e| e.to_string())?;
    save_to_file(res, &filepath)?;

    println!(
        "downloaded input for day {} to file {}",
        day,
        filepath.to_str().unwrap_or("<INVALID_PATH>"),
    );
    Ok(())
}

fn download_description(day: u8, session_token: &str) -> Result<(), String> {
    let mut filepath = PathBuf::new();
    let filename = format!("day{:0>2}-{}.md", day, YEAR);
    filepath.push(DESC_DIR);
    filepath.push(filename);
    if filepath.exists() {
        println!(
            "the file {} already exists. skipping download.",
            filepath.to_str().unwrap_or("<INVALID_PATH>"),
        );
        return Ok(());
    }

    let url = format!("https://adventofcode.com/{}/day/{}", YEAR, day);
    let res = get_response(url, session_token).map_err(|e| e.to_string())?;
    let md = html2md::parse_html(&res);
    save_to_file(md, &filepath)?;

    println!(
        "downloaded description for day {} to file {}",
        day,
        filepath.to_str().unwrap_or("<INVALID_PATH>"),
    );
    Ok(())
}

pub fn download(day: u8) -> Result<(), String> {
    dotenv().ok();
    let session_token = env::var("SESSION_TOKEN").map_err(
        |_| "Couldn't find a session token. Make sure a SESSION_TOKEN environment variable exists.",
    )?;

    download_input(day, &session_token)?;
    download_description(day, &session_token)
}
