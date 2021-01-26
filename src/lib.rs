use chrono::{DateTime, Utc};
use serde::{de::Error, Deserialize, Deserializer};

pub struct Client;

impl Client {
    pub fn pulls(owner: impl Into<String>, repo: impl Into<String>) -> PullsHandler {
        PullsHandler {
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    pub fn activity() -> ActivityHandler {
        ActivityHandler {}
    }
}

pub struct PullsHandler {
    owner: String,
    repo: String,
}

impl PullsHandler {
    pub fn list(&self) -> Result<Vec<PullRequest>, ureq::Error> {
        let pull_requests: Vec<PullRequest> = ureq::get(&format!(
            "https://api.github.com/repos/{}/{}/pulls",
            self.owner, self.repo
        ))
        .call()?
        .into_json()?;
        Ok(pull_requests)
    }
}

pub struct ActivityHandler;

impl ActivityHandler {
    pub fn events(&self) -> EventHandler {
        EventHandler {}
    }
}

pub struct EventHandler;

impl EventHandler {
    pub fn list_user_events(&self, user: impl Into<String>) -> Result<Vec<Event>, ureq::Error> {
        let user_events: Vec<Event> = ureq::get(&format!(
            "https://api.github.com/users/{}/events",
            user.into()
        ))
        .call()?
        .into_json()?;
        Ok(user_events)
    }
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub number: u64,
    pub state: String,
    pub title: String,
    pub body: String,
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub requested_reviewers: Vec<User>,
    // pub requested_teams: ,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    pub locked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_commit_sha: Option<String>,
    // head:,
    // base:,
    // links:,
    pub author_association: String,
    pub draft: bool,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
}

#[derive(Debug, Deserialize)]
pub struct Milestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub state: String,
    pub title: String,
    pub description: String,
    pub creator: User,
    pub open_issues: i64,
    pub closed_issues: i64,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct Label {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub color: String,
    pub default: bool,
}

#[derive(Debug)]
pub struct Event {
    pub id: String,
    pub r#type: EventType,
    pub actor: Actor,
    pub repo: Repository,
    pub public: bool,
    pub created_at: DateTime<Utc>,
    pub payload: Option<Payload>,
    pub org: Option<Org>,
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Tmp {
            id: String,
            r#type: EventType,
            actor: Actor,
            repo: Repository,
            public: bool,
            created_at: DateTime<Utc>,
            payload: Option<serde_json::Value>,
            org: Option<Org>,
        }
        let tmp = Tmp::deserialize(deserializer)?;
        let payload = tmp.payload.clone().map_or(Ok(None), |data| {
            deserialize_payload(&tmp.r#type, data).map_err(|e| Error::custom(e.to_string()))
        })?;
        let event = Event {
            id: tmp.id,
            r#type: tmp.r#type,
            actor: tmp.actor,
            repo: tmp.repo,
            public: tmp.public,
            created_at: tmp.created_at,
            payload,
            org: tmp.org,
        };
        Ok(event)
    }
}

fn deserialize_payload(
    event_type: &EventType,
    data: serde_json::Value,
) -> Result<Option<Payload>, serde_json::Error> {
    let payload = match event_type {
        EventType::IssuesEvent => Some(
            serde_json::from_value::<IssuesEventPayload>(data).map(Payload::IssuesEventPayload)?,
        ),
        EventType::PullRequestEvent => Some(
            serde_json::from_value::<PullRequestEventPayload>(data)
                .map(Payload::PullRequestEventPayload)?,
        ),
        EventType::PullRequestReviewCommentEvent => Some(
            serde_json::from_value::<PullRequestReviewCommentEventPayload>(data)
                .map(Payload::PullRequestReviewCommentEventPayload)?,
        ),
        EventType::IssueCommentEvent => Some(
            serde_json::from_value::<IssueCommentEventPayload>(data)
                .map(Payload::IssueCommentEventPayload)?,
        ),
        EventType::CommitCommentEvent => Some(
            serde_json::from_value::<CommitCommentEventPayload>(data)
                .map(Payload::CommitCommentEventPayload)?,
        ),
        _ => None,
    };
    Ok(payload)
}

#[derive(Debug, Deserialize)]
pub enum EventType {
    IssuesEvent,
    PullRequestEvent,
    PullRequestReviewCommentEvent,
    IssueCommentEvent,
    CommitCommentEvent,
    #[serde(other)]
    UnknownEvent,
}

#[derive(Debug, Deserialize)]
pub enum Payload {
    IssuesEventPayload(IssuesEventPayload),
    PullRequestEventPayload(PullRequestEventPayload),
    PullRequestReviewCommentEventPayload(PullRequestReviewCommentEventPayload),
    IssueCommentEventPayload(IssueCommentEventPayload),
    CommitCommentEventPayload(CommitCommentEventPayload),
}

#[derive(Debug, Deserialize)]
pub struct IssuesEventPayload {
    pub action: String,
    pub issue: Issue,
    // pub changes: Changes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestEventPayload {
    pub action: String,
    pub number: u64,
    // pub changes: Changes,
    pub pull_request: PullRequest,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestReviewCommentEventPayload {
    pub action: String,
    // pub changes: Changes,
    pub pull_request: PullRequest,
    pub comment: Comment,
}

#[derive(Debug, Deserialize)]
pub struct IssueCommentEventPayload {
    pub action: String,
    // pub changes: Changes,
    pub issue: Issue,
    pub comment: Comment,
}

#[derive(Debug, Deserialize)]
pub struct CommitCommentEventPayload {
    pub action: String,
    pub comment: Comment,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Actor {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Org {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub number: i64,
    pub state: String,
    pub title: String,
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    pub user: User,
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub author_association: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    pub locked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub comments: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequestLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestLink {
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub id: u64,
    pub node_id: String,
    pub url: String,
    pub html_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    pub user: User,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}
