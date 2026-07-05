use reqwest::blocking::Client;
use std::fs::{create_dir_all, read_to_string, write};

pub fn input(year: u32, day: u32) -> String {
    let cache_path = format!(".cache/{year}-{day:02}.txt");

    if let Ok(cached) = read_to_string(&cache_path) {
        return cached;
    }

    let input = download_puzzle_input(year, day);

    create_dir_all(".cache/").ok();
    write(&cache_path, &input).unwrap();

    input
}

fn download_puzzle_input(year: u32, day: u32) -> String {
    let session = read_to_string(".session")
        .expect("Please provide a valid session cookie in .session in the project root.");
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let response = Client::new()
        .get(url)
        // https://www.reddit.com/r/adventofcode/comments/z9dhtd/please_include_your_contact_info_in_the_useragent/
        .header(
            "User-Agent",
            "https://github.com/pindab0ter/advent-of-code-rust",
        )
        .header("From", "hansvanluttikhuizen@me.com")
        .header("Cookie", format!("session={session}"))
        .send()
        .unwrap();

    let status = response.status();
    let text = response.text().unwrap().trim().to_owned();

    if status.is_client_error() {
        panic!(
            "Invalid session cookie. Please provide a valid session cookie in .session in the project root."
        )
    } else if !status.is_success() {
        panic!("Unexpected status code: {}\n{}", status, text)
    }

    text.to_string()
}
