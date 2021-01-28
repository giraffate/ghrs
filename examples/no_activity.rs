use ghrs::Client;

use chrono::{Duration, Utc};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = std::env::args().collect();
    let owner = args.get(1).unwrap();
    let repo = args.get(2).unwrap();

    let pull_requests = Client::pulls(owner, repo).list().per_page(100).send()?;

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
