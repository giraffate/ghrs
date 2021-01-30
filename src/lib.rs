pub mod events;
pub mod issues;
pub mod model;
pub mod pulls;

use crate::events::EventsHandler;
use crate::issues::IssuesHandler;
use crate::pulls::PullsHandler;

pub struct Client;

impl Client {
    pub fn issues(owner: impl Into<String>, repo: impl Into<String>) -> IssuesHandler {
        IssuesHandler::new(owner, repo)
    }

    pub fn pulls(owner: impl Into<String>, repo: impl Into<String>) -> PullsHandler {
        PullsHandler::new(owner, repo)
    }

    pub fn events() -> EventsHandler {
        EventsHandler::new()
    }
}

#[derive(Debug)]
pub struct Page<T> {
    items: Vec<T>,
    prev: Option<String>,
    next: Option<String>,
}

impl<T: serde::de::DeserializeOwned> Page<T> {
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
    pub fn get_next_page(&self) -> Option<Page<T>> {
        if let Some(next) = self.next.clone() {
            let response = ureq::get(next.as_str()).call().unwrap();
            let page: Page<T> = Page::from_response(response).unwrap();
            Some(page)
        } else {
            None
        }
    }

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
}
