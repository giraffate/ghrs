use crate::model::Issue;
use crate::Page;

/// A client for the Issue API.
///
/// See <https://docs.github.com/en/rest/reference/issues>.
pub struct IssuesHandler {
    owner: String,
    repo: String,
}

impl IssuesHandler {
    pub fn new(owner: impl Into<String>, repo: impl Into<String>) -> IssuesHandler {
        IssuesHandler {
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// List issues.
    ///
    /// See <https://docs.github.com/en/rest/reference/issues#list-repository-issues>.
    /// ```no_run
    /// let client = ghrs::Client::new();
    /// let issues = client.pulls("owner", "repo").list().per_page(100).send();
    /// ```
    pub fn list(&self) -> ListIssuesBuilder {
        ListIssuesBuilder::new(&self)
    }

    /// Get an issue.
    ///
    /// See <hhttps://docs.github.com/en/rest/reference/issues#get-an-issue>.
    /// ```no_run
    /// let client = ghrs::Client::new();
    /// let issue = client.issues("owner", "repo").get(1234).send();
    /// ```
    pub fn get(&self, issue_number: u64) -> GetIssueBuilder {
        GetIssueBuilder::new(&self, issue_number)
    }
}

/// A builder for listing issues.
pub struct ListIssuesBuilder<'a> {
    handler: &'a IssuesHandler,
    accept: Option<String>,
    milestone: Option<String>,
    state: Option<String>,
    assignee: Option<String>,
    creator: Option<String>,
    mentioned: Option<String>,
    labels: Option<String>,
    sort: Option<String>,
    direction: Option<String>,
    since: Option<String>,
    per_page: Option<u8>,
    page: Option<u8>,
}

impl<'a> ListIssuesBuilder<'a> {
    pub fn new(handler: &'a IssuesHandler) -> Self {
        ListIssuesBuilder {
            handler,
            accept: None,
            milestone: None,
            state: None,
            assignee: None,
            creator: None,
            mentioned: None,
            labels: None,
            sort: None,
            direction: None,
            since: None,
            per_page: None,
            page: None,
        }
    }

    /// List issues.
    ///
    /// See <https://docs.github.com/en/rest/reference/issues#list-repository-issues>.
    /// ```no_run
    /// let client = ghrs::Client::new();
    /// let issues = client.issues("owner", "repo").list().per_page(100).send();
    /// ```
    pub fn send(&self) -> Result<Page<Issue>, ureq::Error> {
        let mut request = ureq::get(&format!(
            "https://api.github.com/repos/{}/{}/issues",
            self.handler.owner, self.handler.repo
        ));

        if let Some(accept) = self.accept.clone() {
            request = request.set("Accept", &accept);
        }
        if let Some(milestone) = self.milestone.clone() {
            request = request.query("milestone", &milestone);
        }
        if let Some(state) = self.state.clone() {
            request = request.query("state", &state);
        }
        if let Some(assignee) = self.assignee.clone() {
            request = request.query("assignee", &assignee);
        }
        if let Some(creator) = self.creator.clone() {
            request = request.query("creator", &creator);
        }
        if let Some(mentioned) = self.mentioned.clone() {
            request = request.query("mentioned", &mentioned);
        }
        if let Some(labels) = self.labels.clone() {
            request = request.query("labels", &labels);
        }
        if let Some(sort) = self.sort.clone() {
            request = request.query("sort", &sort);
        }
        if let Some(direction) = self.direction.clone() {
            request = request.query("direction", &direction);
        }
        if let Some(since) = self.since.clone() {
            request = request.query("milestone", &since);
        }
        if let Some(per_page) = self.per_page {
            request = request.query("per_page", &per_page.to_string());
        }
        if let Some(page) = self.page {
            request = request.query("page", &page.to_string());
        }

        let response = request.call()?;
        let issues = Page::from_response(response)?;
        Ok(issues)
    }

    pub fn accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    pub fn milestone(mut self, milestone: impl Into<String>) -> Self {
        self.milestone = Some(milestone.into());
        self
    }

    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    pub fn creator(mut self, creator: impl Into<String>) -> Self {
        self.creator = Some(creator.into());
        self
    }

    pub fn mentioned(mut self, mentioned: impl Into<String>) -> Self {
        self.mentioned = Some(mentioned.into());
        self
    }

    pub fn labels(mut self, labels: impl Into<String>) -> Self {
        self.labels = Some(labels.into());
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

    pub fn since(mut self, since: impl Into<String>) -> Self {
        self.since = Some(since.into());
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

/// A builder for getting an issue.
pub struct GetIssueBuilder<'a> {
    handler: &'a IssuesHandler,
    issue_number: u64,
    accept: Option<String>,
}

impl<'a> GetIssueBuilder<'a> {
    fn new(handler: &'a IssuesHandler, issue_number: u64) -> Self {
        GetIssueBuilder {
            handler,
            issue_number,
            accept: None,
        }
    }

    /// Get an issue.
    ///
    /// See <https://docs.github.com/en/rest/reference/issues#get-an-issue>.
    /// ```no_run
    /// let client = ghrs::Client::new();
    /// let issue = client.issues("owner", "repo").get(1234).send();
    /// ```
    pub fn send(&self) -> Result<Issue, ureq::Error> {
        let mut request = ureq::get(&format!(
            "https://api.github.com/repos/{}/{}/issues/{}",
            self.handler.owner, self.handler.repo, self.issue_number
        ));

        if let Some(accept) = self.accept.clone() {
            request = request.set("Accept", &accept);
        }

        let issue: Issue = request.call()?.into_json()?;
        Ok(issue)
    }

    pub fn accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }
}
