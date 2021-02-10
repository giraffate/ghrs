//! The Pull Request API
use crate::model::PullRequest;
use crate::{Client, Page};

/// A client for the Pull Request API.
///
/// See <https://docs.github.com/en/rest/reference/pulls>.
pub struct PullsHandler<'a> {
    client: &'a Client,
    owner: String,
    repo: String,
}

impl<'a> PullsHandler<'a> {
    pub fn new(
        client: &'a Client,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> PullsHandler {
        PullsHandler {
            client,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// List pull requests.
    ///
    /// See <https://docs.github.com/en/rest/reference/pulls#list-pull-requests>.
    /// ```no_run
    /// let client = ghrs::Client::new();
    /// let pull_requests = client.pulls("owner", "repo").list().per_page(100).send();
    /// ```
    pub fn list(&self) -> ListPullRequestsBuilder {
        ListPullRequestsBuilder::new(&self)
    }

    /// Get a pull request.
    ///
    /// See <https://docs.github.com/en/rest/reference/pulls#get-a-pull-request>.
    /// ```no_run
    /// let client = ghrs::Client::new();
    /// let pull_request = client.pulls("owner", "repo").get(1234).send();
    /// ```
    pub fn get(&self, pull_number: u64) -> GetPullRequestBuilder {
        GetPullRequestBuilder::new(&self, pull_number)
    }
}

/// A builder for listing pull requests.
pub struct ListPullRequestsBuilder<'a> {
    handler: &'a PullsHandler<'a>,
    accept: Option<String>,
    per_page: Option<u8>,
    page: Option<u8>,
}

impl<'a> ListPullRequestsBuilder<'a> {
    pub fn new(handler: &'a PullsHandler) -> Self {
        ListPullRequestsBuilder {
            handler,
            accept: None,
            per_page: None,
            page: None,
        }
    }

    /// List pull requests.
    ///
    /// See <https://docs.github.com/en/rest/reference/pulls#list-pull-requests>.
    /// ```no_run
    /// let client = ghrs::Client::new();
    /// let pull_requests = client.pulls("owner", "repo").list().per_page(100).send();
    /// ```
    pub fn send(&self) -> Result<Page<PullRequest>, ureq::Error> {
        let mut request = ureq::get(&format!(
            "https://api.github.com/repos/{}/{}/pulls",
            self.handler.owner, self.handler.repo
        ));

        if let Some(token) = self.handler.client.token.clone() {
            request = request.set("Authorization", &format!("token {}", token));
        }
        if let Some(accept) = self.accept.clone() {
            request = request.set("Accept", &accept);
        }
        if let Some(per_page) = self.per_page {
            request = request.query("per_page", &per_page.to_string());
        }
        if let Some(page) = self.page {
            request = request.query("page", &page.to_string());
        }

        let response = request.call()?;
        let pull_requests = Page::from_response(response)?;
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

/// A builder for getting a pull request.
pub struct GetPullRequestBuilder<'a> {
    handler: &'a PullsHandler<'a>,
    pull_number: u64,
    accept: Option<String>,
}

impl<'a> GetPullRequestBuilder<'a> {
    fn new(handler: &'a PullsHandler, pull_number: u64) -> Self {
        GetPullRequestBuilder {
            handler,
            pull_number,
            accept: None,
        }
    }

    /// Get a pull request.
    ///
    /// See <https://docs.github.com/en/rest/reference/pulls#get-a-pull-request>.
    /// ```no_run
    /// let client = ghrs::Client::new();
    /// let pull_request = client.pulls("owner", "repo").get(1234).send();
    /// ```
    pub fn send(&self) -> Result<PullRequest, ureq::Error> {
        let mut request = ureq::get(&format!(
            "https://api.github.com/repos/{}/{}/pulls/{}",
            self.handler.owner, self.handler.repo, self.pull_number
        ));

        if let Some(token) = self.handler.client.token.clone() {
            request = request.set("Authorization", &format!("token {}", token));
        }
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
}
