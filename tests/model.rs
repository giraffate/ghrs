use ghrs::model::{Issue, PullRequest};

#[test]
fn pull_request_de_test() {
    let _: PullRequest = serde_json::from_str(include_str!("models/pull_request.json")).unwrap();
}

#[test]
fn issue_de_test() {
    let _: Issue = serde_json::from_str(include_str!("models/issue.json")).unwrap();
}
