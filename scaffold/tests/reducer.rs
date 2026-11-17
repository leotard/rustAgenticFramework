//! Integration test for the pure core (factor 12): no mocks, no I/O.

use my_project::core::{reduce, Event, Step, Thread};

#[test]
fn reduce_appends_events_and_is_pure() {
    let start = Thread::default();
    let mid = reduce(
        start.clone(),
        Event::Message {
            from: "test".into(),
            text: "hi".into(),
        },
    );
    let end = reduce(
        mid,
        Event::Step {
            step: Step::ListGitTags,
        },
    );

    assert_eq!(start.events.len(), 0);
    assert_eq!(end.events.len(), 2);
}

#[test]
fn thread_round_trips_through_json() {
    let thread = reduce(
        Thread::default(),
        Event::Step {
            step: Step::ListGitTags,
        },
    );
    let json = thread.to_json().unwrap();
    let back: Thread = serde_json::from_str(&json).unwrap();
    assert_eq!(back.events.len(), 1);
}
