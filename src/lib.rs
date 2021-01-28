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

    pub fn events() -> EventHandler {
        EventHandler {
            accept: None,
            per_page: None,
            page: None,
        }
    }
}

/// A client for the Pull Request API.
///
/// See <https://docs.github.com/en/rest/reference/pulls>.
pub struct PullsHandler {
    owner: String,
    repo: String,
    accept: Option<String>,
    per_page: Option<u8>,
    page: Option<u8>,
}

impl PullsHandler {
    /// List pull requests.
    ///
    /// See <https://docs.github.com/en/rest/reference/pulls#list-pull-requests>.
    /// ```no_run
    /// let pull_requests = ghrs::Client::pulls("owner", "repo").list();
    /// ```
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

    /// Get a pull request.
    ///
    /// See <https://docs.github.com/en/rest/reference/pulls#get-a-pull-request>.
    /// ```no_run
    /// let pull_request = ghrs::Client::pulls("owner", "repo").get(1234);
    /// ```
    pub fn get(&self, pull_number: u64) -> Result<PullRequest, ureq::Error> {
        let mut request = ureq::get(&format!(
            "https://api.github.com/repos/{}/{}/pulls/{}",
            self.owner, self.repo, pull_number
        ));

        if let Some(accept) = self.accept.clone() {
            request = request.set("Accept", &accept);
        }

        let pull_request: PullRequest = request.call()?.into_json()?;
        Ok(pull_request)
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

/// A client for the Event API.
///
/// See <https://docs.github.com/en/rest/reference/activity#events>.
pub struct EventHandler {
    accept: Option<String>,
    per_page: Option<u8>,
    page: Option<u8>,
}

impl EventHandler {
    /// List events for the authenticated user.
    ///
    /// See <https://docs.github.com/en/rest/reference/activity#list-events-for-the-authenticated-user>.
    /// ```no_run
    /// let events = ghrs::Client::events().list_user_events("user");
    /// ```
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
