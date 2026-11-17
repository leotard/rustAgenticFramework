//! Integration test for the software factory: PRs open automatically, the
//! merge is gated on human approval.

use my_project::factory::{run_pipeline, Feature, StubGitHub};
use my_project::runtime::ControlFlow;

#[test]
fn pipeline_opens_pr_then_pauses_before_merge() {
    let mut feature = Feature::new("drink-reminders", "Drink reminders");
    let stub = StubGitHub;

    // First pass: research → design → outline → implement → open PR → pause at merge.
    let flow = run_pipeline(&mut feature, &stub).unwrap();
    assert_eq!(flow, ControlFlow::AwaitHuman);
    assert!(
        feature.pr_url.is_some(),
        "PR should be open before the merge gate"
    );
    assert!(!feature.merge_approved);

    // Approve and resume: the merge completes and the pipeline finishes.
    feature.approve_merge();
    let flow = run_pipeline(&mut feature, &stub).unwrap();
    assert_eq!(flow, ControlFlow::Done);
}
