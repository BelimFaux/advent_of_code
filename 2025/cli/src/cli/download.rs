use core::fmt;
use dotenv::dotenv;
use std::{env, error::Error};

const YEAR: u32 = 2024; // set to 2024 for debugging; 2025 will be available in december

#[derive(Debug)]
struct DownloadError(String);

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Download Error: {}", self.0)
    }
}

impl Error for DownloadError {}

fn get_response(url: String, session_token: String) -> Result<String, DownloadError> {
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

pub fn download(day: u8) -> Result<(), String> {
    dotenv().ok();
    let session_token = env::var("SESSION_TOKEN").map_err(
        |_| "Couldn't find a session token. Make sure a SESSION_TOKEN environment variable exists.",
    )?;

    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);
    let res = get_response(url, session_token).map_err(|e| e.to_string())?;
    println!("{}", res);

    Ok(())
}
