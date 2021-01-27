use ghrs::Client;

use chrono::{Duration, Utc};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pull_requests = Client::pulls("rust-lang", "rust-clippy").list()?;

    let earlier_than = Utc::now() - Duration::days(14);
    let pull_requests = pull_requests
        .iter()
        .filter(|x| x.updated_at.unwrap() < earlier_than);
    println!("## Triaged Pull Requests");
    for pull_request in pull_requests {
        println!("- [{}]({})", pull_request.title, pull_request.html_url);
    }
    Ok(())
}
