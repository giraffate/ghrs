use ghrs::model::PullRequest;

#[test]
fn pull_request_de_test() {
    let _: PullRequest = serde_json::from_str(include_str!("models/pull_request.json")).unwrap();
}
