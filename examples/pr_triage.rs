use ghrs::Client;

use chrono::{Duration, Utc};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = std::env::args().collect();
    let owner = args.get(1).unwrap();
    let repo = args.get(2).unwrap();

    let client = Client::new();
    let mut current_page = client
        .pulls(owner, repo)
        .list()
        .per_page(100)
        .page(1)
        .send()?;

    let mut pull_requests = current_page.take_items();

    while let Some(next_page) = current_page.get_next_page() {
        current_page = next_page;
        pull_requests.extend(current_page.take_items());
    }

    let earlier_than = Utc::now() - Duration::days(14);
    let pull_requests = pull_requests
        .into_iter()
        .filter(|x| x.updated_at.unwrap() < earlier_than);
    println!("## Triaged Pull Requests");
    for pull_request in pull_requests {
        println!("- [{}]({})", pull_request.title, pull_request.html_url);
    }

    Ok(())
}
