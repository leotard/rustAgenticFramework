//! Factor 11 — webhook trigger. Start with `cargo run -- webhook`, then POST to
//! `http://localhost:3000/webhook` with `{"thread_id": "...", "from": "...", "text": "..."}`.

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;

use super::handle_event;
use crate::agents::deploy::DeployAgent;
use crate::core::{Event, FileStore, ScriptedEngine, Step};
use crate::tools::git::GitTools;

#[derive(Deserialize)]
pub struct Inbound {
    pub thread_id: String,
    pub from: String,
    pub text: String,
}

pub fn router() -> Router {
    Router::new().route("/webhook", post(webhook))
}

async fn webhook(Json(payload): Json<Inbound>) -> impl IntoResponse {
    let store = FileStore::new(".agents/state");
    let agent = DeployAgent;
    // TODO: route by content to the right specialist agent (factor 10: triage).
    let engine = ScriptedEngine::new(vec![
        Step::ListGitTags,
        Step::DoneForNow {
            message: "triaged".into(),
        },
    ]);
    let tools = GitTools;

    let event = Event::Message {
        from: payload.from,
        text: payload.text,
    };

    match handle_event(&store, &payload.thread_id, event, &agent, &engine, &tools) {
        Ok(_) => (StatusCode::OK, "ok").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn serve() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router()).await?;
    Ok(())
}
