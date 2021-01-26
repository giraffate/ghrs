pub mod model;

use crate::model::{Event, PullRequest};

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
