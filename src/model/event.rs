use super::{Issue, PullRequest, Comment, Label, User};

use chrono::{DateTime, Utc};
use serde::{de::Error, Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum EventType {
    IssuesEvent,
    PullRequestEvent,
    PullRequestReviewCommentEvent,
    IssueCommentEvent,
    CommitCommentEvent,
    #[serde(other)]
    UnknownEvent,
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Payload {
    IssuesEventPayload(IssuesEventPayload),
    PullRequestEventPayload(PullRequestEventPayload),
    PullRequestReviewCommentEventPayload(PullRequestReviewCommentEventPayload),
    IssueCommentEventPayload(IssueCommentEventPayload),
    CommitCommentEventPayload(CommitCommentEventPayload),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuesEventPayload {
    pub action: String,
    pub issue: Issue,
    // pub changes: Changes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PullRequestEventPayload {
    pub action: String,
    pub number: u64,
    // pub changes: Changes,
    pub pull_request: PullRequest,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PullRequestReviewCommentEventPayload {
    pub action: String,
    // pub changes: Changes,
    pub pull_request: PullRequest,
    pub comment: Comment,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssueCommentEventPayload {
    pub action: String,
    // pub changes: Changes,
    pub issue: Issue,
    pub comment: Comment,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommitCommentEventPayload {
    pub action: String,
    pub comment: Comment,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Actor {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Org {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}
