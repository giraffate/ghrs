pub mod model;

use crate::model::{Event, PullRequest};

pub struct Client;

impl Client {
    pub fn pulls(owner: impl Into<String>, repo: impl Into<String>) -> PullsHandler {
        PullsHandler {
            owner: owner.into(),
            repo: repo.into(),
            accept: None,
            per_page: None,
            page: None,
        }
    }

    pub fn activity() -> ActivityHandler {
        ActivityHandler {}
    }
}

pub struct PullsHandler {
    owner: String,
    repo: String,
    accept: Option<String>,
    per_page: Option<u8>,
    page: Option<u8>,
}

impl PullsHandler {
    pub fn list(&self) -> Result<Vec<PullRequest>, ureq::Error> {
        let mut request = ureq::get(&format!(
            "https://api.github.com/repos/{}/{}/pulls",
            self.owner, self.repo
        ));

        if let Some(accept) = self.accept.clone() {
            request = request.set("Accept", &accept);
        }
        if let Some(per_page) = self.per_page {
            request = request.query("per_page", &per_page.to_string());
        }
        if let Some(page) = self.page {
            request = request.query("page", &page.to_string());
        }

        let pull_requests: Vec<PullRequest> = request.call()?.into_json()?;
        Ok(pull_requests)
    }

    pub fn accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }
}

pub struct ActivityHandler;

impl ActivityHandler {
    pub fn events(&self) -> EventHandler {
        EventHandler {
            accept: None,
            per_page: None,
            page: None,
        }
    }
}

pub struct EventHandler {
    accept: Option<String>,
    per_page: Option<u8>,
    page: Option<u8>,
}

impl EventHandler {
    pub fn list_user_events(&self, user: impl Into<String>) -> Result<Vec<Event>, ureq::Error> {
        let mut request = ureq::get(&format!(
            "https://api.github.com/users/{}/events",
            user.into()
        ));

        if let Some(accept) = self.accept.clone() {
            request = request.set("Accept", &accept);
        }
        if let Some(per_page) = self.per_page {
            request = request.query("per_page", &per_page.to_string());
        }
        if let Some(page) = self.page {
            request = request.query("page", &page.to_string());
        }

        let user_events: Vec<Event> = request.call()?.into_json()?;
        Ok(user_events)
    }

    pub fn accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }
}
