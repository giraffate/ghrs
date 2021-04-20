# ghrs
ghrs is a simple client for GitHub v3 API. This has a simple interface and blocking I/O, it avoids complexity unlike Async I/O, so it's so easy to use. ghrs is inspired by [Octocrab](https://github.com/XAMPPRocky/octocrab).

The following modules are available now.
- [`issues`](https://docs.rs/ghrs/latest/ghrs/issues/struct.IssuesHandler.html)
- [`pulls`](https://docs.rs/ghrs/latest/ghrs/pulls/struct.PullsHandler.html)
- [`events`](https://docs.rs/ghrs/latest/ghrs/events/struct.EventsHandler.html)

## Usage
[List pull requests](https://docs.github.com/en/rest/reference/pulls#list-pull-requests).
```rust
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let mut current_page = client
        .pulls("owner", "repo")
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

### GitHub Enterprise
If you use `ghrs` for GitHub Enterprise, set `base_url`.
```rust
// GET `https://github.your_company.com/api/v3/repos/owner/repo/issues`
let client = ghrs::Client::new();
let mut current_page = client
    .base_url("https://github.your_company.com/api/v3")
    .token("your_token")
    .issues("owner", "repo")
    .list()
    .send()?;
```

## Contributing
1. Fork
2. Create a feature branch
3. Commit your changes
4. Rebase your local changes against the master branch
5. Run test suite with the `cargo test` command and confirm that it passes
6. Run `cargo fmt` and pass `cargo clippy`
7. Create new Pull Request

## License
[MIT license](LICENSE)
