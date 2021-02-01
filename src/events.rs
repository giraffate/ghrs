use crate::model::Event;
use crate::Page;

/// A client for the Event API.
///
/// See <https://docs.github.com/en/rest/reference/activity#events>.
pub struct EventsHandler;

impl EventsHandler {
    pub fn new() -> EventsHandler {
        EventsHandler {}
    }

    pub fn list_user_events(&self, user: impl Into<String>) -> ListUserEventsBuilder {
        ListUserEventsBuilder::new(user)
    }
}

/// A builder for listing user events
pub struct ListUserEventsBuilder {
    user: String,
    accept: Option<String>,
    per_page: Option<u8>,
    page: Option<u8>,
}

impl ListUserEventsBuilder {
    fn new(user: impl Into<String>) -> ListUserEventsBuilder {
        ListUserEventsBuilder {
            user: user.into(),
            accept: None,
            per_page: None,
            page: None,
        }
    }

    /// List events for the authenticated user.
    ///
    /// See <https://docs.github.com/en/rest/reference/activity#list-events-for-the-authenticated-user>.
    /// ```no_run
    /// let client = ghrs::Client::new();
    /// let events = client.events().list_user_events("user").per_page(100).send();
    /// ```
    pub fn send(&self) -> Result<Page<Event>, ureq::Error> {
        let mut request = ureq::get(&format!(
            "https://api.github.com/users/{}/events",
            self.user
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

        let response = request.call()?;
        let user_events = Page::from_response(response)?;
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
