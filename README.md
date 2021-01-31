# ghrs
ghrs is a simple client for GitHub v3 API. This has a simple interface and blocking I/O, it avoids complexity unlike Async I/O, so it's so easy to use. ghrs is inspired by [Octocrab](https://github.com/XAMPPRocky/octocrab).

## Usage
[List pull requests](https://docs.github.com/en/rest/reference/pulls#list-pull-requests).
```rust
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut current_page = Client::pulls("owner", "repo")
        .list()
        .per_page(100)
        .page(1)
        .send()?;

    // You get pull requests.
    let mut pull_requests = current_page.take_items();

    // If you want to get next pages, see here.
    while let Some(next_page) = current_page.get_next_page() {
        current_page = next_page;
        pull_requests.extend(current_page.take_items());
    }

    Ok(())
}
```

## Contributing
## License
[MIT license](LICENSE)
