use ghrs::Client;

use chrono::{Duration, Utc};
use ureq::Error;

fn main() {
    match run() {
        Ok(_) => {}
        Err(Error::Status(_code, status)) => {
            println!("Request failed: {:?}", status);
        }
        Err(Error::Transport(transport)) => {
            println!("{}", transport);
        }
    }
}

fn run() -> Result<(), ureq::Error> {
    let body = Client::pulls("rust-lang", "rust-clippy").list()?;
    let earlier_than = Utc::now() - Duration::days(14);
    let titles = body
        .iter()
        .filter(|x| x.updated_at.unwrap() < earlier_than)
        .map(|x| x.title.clone());
    for title in titles {
        println!("{}", title);
    }
    Ok(())
}
