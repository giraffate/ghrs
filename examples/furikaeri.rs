use ghrs::{model::EventType, model::Payload, Client};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let events = Client::activity().events().list_user_events("giraffate")?;

    let mut issues_events = Vec::new();
    let mut pull_request_events = Vec::new();
    let mut pull_request_review_comment_events = Vec::new();
    let mut issue_comment_events = Vec::new();
    let mut commit_comment_events = Vec::new();
    for event in events.into_iter() {
        match event.r#type {
            EventType::IssuesEvent => issues_events.push(event),
            EventType::PullRequestEvent => pull_request_events.push(event),
            EventType::PullRequestReviewCommentEvent => {
                pull_request_review_comment_events.push(event)
            }
            EventType::IssueCommentEvent => issue_comment_events.push(event),
            EventType::CommitCommentEvent => commit_comment_events.push(event),
            _ => {}
        }
    }

    println!("## Issues Event");
    for event in issues_events {
        if let Payload::IssuesEventPayload(payload) = event.payload.unwrap() {
            println!("- [{}]({})", payload.issue.title, payload.issue.html_url);
        }
    }
    println!("");
    println!("## Pull Request Event");
    for event in pull_request_events {
        if let Payload::PullRequestEventPayload(payload) = event.payload.unwrap() {
            println!(
                "- [{}][{}]",
                payload.pull_request.title, payload.pull_request.html_url
            );
        }
    }
    println!("");
    println!("## Pull Request Review Comment Event");
    for event in pull_request_review_comment_events {
        if let Payload::PullRequestReviewCommentEventPayload(payload) = event.payload.unwrap() {
            println!(
                "- [{}][{}]",
                payload.pull_request.title, payload.comment.html_url
            );
        }
    }
    println!("");
    println!("## Issue Comment Event");
    for event in issue_comment_events {
        if let Payload::IssueCommentEventPayload(payload) = event.payload.unwrap() {
            println!("- [{}][{}]", payload.issue.title, payload.comment.html_url);
        }
    }
    println!("");
    println!("## Commit Comment Event");
    for event in commit_comment_events {
        if let Payload::CommitCommentEventPayload(payload) = event.payload.unwrap() {
            println!(
                "- [{}][{}]",
                payload.comment.html_url, payload.comment.html_url
            );
        }
    }
    println!("");
    Ok(())
}
