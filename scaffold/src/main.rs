use my_project::{reduce, Event, Thread};

fn main() {
    let thread = Thread::default().push(Event::Message {
        from: "cli".to_string(),
        text: "hello".to_string(),
    });

    let next = reduce(
        thread,
        Event::ToolCall {
            intent: "done_for_now".to_string(),
            arguments: serde_json::json!({}),
        },
    );

    println!("events: {}", next.events.len());
}
