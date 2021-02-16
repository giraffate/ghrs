use ghrs::Client;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let args: Vec<String> = std::env::args().collect();
    let base_url = args.get(1).unwrap();
    let token = args.get(2).unwrap();
    let owner = args.get(3).unwrap();
    let repo = args.get(4).unwrap();

    let client = Client::new();
    let mut current_page = client.base_url(base_url).token(token).issues(owner, repo).list().per_page(100).page(1).send()?;
    let issues = current_page.take_items();

    for issue in issues {
      println!("{}", issue.title);
    }

    Ok(())
}
