use std::{fs, io};
use std::io::Write;
use std::path::Path;

use reqwest::blocking::Client;
use reqwest::header::{COOKIE, HeaderMap};

fn read_input(msg: &str) -> std::io::Result<String> {
    let mut buffer = String::new();
    print!("{}", msg);
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn send_get(session: &str, url: &str) -> reqwest::Result<String> {
    // Building HTTP Header
    let mut headers = HeaderMap::new();
    headers.append(COOKIE, session.parse().expect("Couldn't parse session"));
    let client = Client::builder().default_headers(headers).build()?;

    // Fetch response
    let res = client.execute(client.get(url).build()?)?;

    Ok(res.text()?.trim().to_string())
}

pub fn extract_integer<T: std::str::FromStr>(s: &str) -> Result<T, &str> {
    let res = s.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<T>();

    match res {
        Ok(v) => Ok(v),
        Err(_) => Err("Couldn't parse string")
    }
}

pub fn get_session() -> String {
    let session_file = Path::new("SESSION");

    if session_file.is_file() {
        // Session file exists, read session key in it
        fs::read_to_string(session_file).expect("Unable to read session file").trim().to_string()
    } else {
        // Session file doesn't exist, ask to user
        let session = read_input("session: ").expect("Unable to read input");

        // Write session in file to avoid asking user everytime
        fs::write(session_file, &session.trim().to_string()).unwrap();

        session.trim().to_string()
    }
}

pub fn get_input(year: u16, day: u8) -> String {
    let filename = format!("./input/{}/day{:02}.txt", year, day);
    let p = Path::new(&filename);

    if p.is_file() {
        fs::read_to_string(filename).expect("Unable to read input file").trim().to_string()
    } else {
        // Building session cookie string
        let mut session = String::from("session=");
        let session_value = get_session();
        session.push_str(&session_value);

        // Fetch input
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let text = send_get(&session, &url).expect("Couldn't fetch input");

        // Writing input inside file to avoid downloading input everytime
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(filename, &text).unwrap();

        text
    }
}