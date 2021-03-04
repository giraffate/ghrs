//! ghrs is a simple client for GitHub v3 API. This has a simple interface and blocking I/O,
//! it avoids complexity unlike Async I/O, so it's so easy to use.
//! ghrs is inspired by [Octocrab](https://github.com/XAMPPRocky/octocrab).

pub mod events;
pub mod issues;
pub mod model;
pub mod pulls;

use crate::events::EventsHandler;
use crate::issues::IssuesHandler;
use crate::pulls::PullsHandler;

const GITHUB_API_URL: &str = "https://api.github.com";

/// A client for GitHub v3 API.
pub struct Client {
    base_url: String,
    token: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Create a `Client`.
    pub fn new() -> Client {
        Client {
            token: None,
            base_url: GITHUB_API_URL.to_string(),
        }
    }

    /// Set a personal access token.
    ///
    /// ```no_run
    /// let client = ghrs::Client::new();
    /// let issues = client.token("your_token").issues("owner", "repo").list().send();
    /// ```
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Set a base URL.
    ///
    /// ```no_run
    /// let client = ghrs::Client::new();
    /// let issues = client.base_url("https://github.your_company.com/api/v3").token("your_token").issues("owner", "repo").list().send();
    /// ```
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Create a [`issues::IssuesHandler`].
    pub fn issues(&self, owner: impl Into<String>, repo: impl Into<String>) -> IssuesHandler {
        IssuesHandler::new(self, owner, repo)
    }

    /// Create a [`pulls::PullsHandler`].
    pub fn pulls(&self, owner: impl Into<String>, repo: impl Into<String>) -> PullsHandler {
        PullsHandler::new(self, owner, repo)
    }

    /// Create a [`events::EventsHandler`].
    pub fn events(&self) -> EventsHandler {
        EventsHandler::new(self)
    }
}

/// A page which enables to get prev/next pages.
#[derive(Debug)]
pub struct Page<T> {
    items: Vec<T>,
    prev: Option<String>,
    next: Option<String>,
}

impl<T: serde::de::DeserializeOwned> Page<T> {
    /// Create a [`Page`] from response.
    pub fn from_response(response: ureq::Response) -> Result<Page<T>, ureq::Error> {
        let link_header = {
            if let Some(link_header) = response.header("link") {
                link_header.parse::<hyperx::header::Link>().unwrap()
            } else {
                let items: Vec<T> = response.into_json()?;
                return Ok(Page {
                    items,
                    prev: None,
                    next: None,
                });
            }
        };

        let mut next = None;
        let mut prev = None;
        for v in link_header.values() {
            let rel = v.rel().unwrap();
            if rel.contains(&hyperx::header::RelationType::Next) {
                next = Some(v.link().to_string());
            }
            if rel.contains(&hyperx::header::RelationType::Prev) {
                prev = Some(v.link().to_string());
            }
        }
        let items: Vec<T> = response.into_json()?;
        Ok(Page { items, prev, next })
    }
}

impl<T> IntoIterator for Page<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T: serde::de::DeserializeOwned> Page<T> {
    /// Returns a next page.
    pub fn get_next_page(&self) -> Option<Page<T>> {
        if let Some(next) = self.next.clone() {
            let response = ureq::get(next.as_str()).call().unwrap();
            let page: Page<T> = Page::from_response(response).unwrap();
            Some(page)
        } else {
            None
        }
    }

    /// Returns current items, and set an empty `Vec` instead.
    pub fn take_items(&mut self) -> Vec<T> {
        std::mem::replace(&mut self.items, Vec::new())
    }

    pub fn get_prev(&self) -> Option<String> {
        self.prev.clone()
    }

    pub fn get_next(&self) -> Option<String> {
        self.next.clone()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
