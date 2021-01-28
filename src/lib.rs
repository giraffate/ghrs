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
