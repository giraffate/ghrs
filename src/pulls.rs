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
    state: Option<String>,
    head: Option<String>,
    base: Option<String>,
    sort: Option<String>,
    direction: Option<String>,
    per_page: Option<u8>,
    page: Option<u8>,
}

impl<'a> ListPullRequestsBuilder<'a> {
    pub fn new(handler: &'a PullsHandler) -> Self {
        ListPullRequestsBuilder {
            handler,
            accept: None,
            state: None,
            head: None,
            base: None,
            sort: None,
            direction: None,
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
            "{}/repos/{}/{}/pulls",
            self.handler.client.base_url, self.handler.owner, self.handler.repo
        ));

        if let Some(token) = self.handler.client.token.clone() {
            request = request.set("Authorization", &format!("token {}", token));
        }
        if let Some(accept) = self.accept.clone() {
            request = request.set("Accept", &accept);
        }
        if let Some(state) = self.state.clone() {
            request = request.query("state", &state);
        }
        if let Some(head) = self.head.clone() {
            request = request.query("head", &head);
        }
        if let Some(base) = self.base.clone() {
            request = request.query("base", &base);
        }
        if let Some(sort) = self.sort.clone() {
            request = request.query("sort", &sort);
        }
        if let Some(direction) = self.direction.clone() {
            request = request.query("direction", &direction);
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

    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn head(mut self, head: impl Into<String>) -> Self {
        self.head = Some(head.into());
        self
    }

    pub fn base(mut self, base: impl Into<String>) -> Self {
        self.base = Some(base.into());
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
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
            "{}/repos/{}/{}/pulls/{}",
            self.handler.client.base_url, self.handler.owner, self.handler.repo, self.pull_number
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
