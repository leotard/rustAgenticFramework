//! Factor 11 — webhook triggers. Start with `cargo run -- webhook`.
//!
//! - `POST /webhook`  — inbound event for the deploy agent loop.
//! - `POST /factory`  — a feature request that fires the software factory.
//!   This is how an orchestrator agent (or a GitHub issue hook) triggers the
//!   pipeline — a human does not type `cargo run -- factory`.

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;

use super::handle_event;
use crate::agents::deploy::DeployAgent;
use crate::core::{Event, FileStore, ScriptedEngine, Step};
use crate::factory::{
    dispatcher_from_choice, dispatcher_from_env, run_pipeline, slugify, Dispatcher, Feature,
    FeatureStore, GitHubCli,
};
use crate::runtime::ControlFlow;
use crate::tools::git::GitTools;

#[derive(Deserialize)]
pub struct Inbound {
    pub thread_id: String,
    pub from: String,
    pub text: String,
}

#[derive(Deserialize)]
pub struct FactoryRequest {
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub agent: Option<String>,
}

pub fn router() -> Router {
    Router::new()
        .route("/webhook", post(webhook))
        .route("/factory", post(factory_hook))
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

/// Fires the software factory for a feature request. The agent is chosen from
/// the request body (`agent`), or falls back to the `RUSTFORGE_AGENT` env var.
async fn factory_hook(Json(payload): Json<FactoryRequest>) -> impl IntoResponse {
    let id = slugify(&payload.title);
    let store = FeatureStore::new(".agents/factory");
    let mut feature = store
        .load(&id)
        .unwrap_or_else(|_| Feature::new(&id, &payload.title));
    feature.description = payload.description;

    let dispatcher: Box<dyn Dispatcher> = match payload.agent {
        Some(a) => dispatcher_from_choice(&a),
        None => dispatcher_from_env(),
    };

    match run_pipeline(&mut feature, &GitHubCli, dispatcher.as_ref()) {
        Ok(flow) => {
            let _ = store.save(&feature);
            let pr = feature.pr_url.as_deref().unwrap_or("<no pr>");
            let body = match flow {
                ControlFlow::Done => format!("merged: {pr}"),
                ControlFlow::AwaitHuman => format!("paused for merge approval: {pr}"),
                ControlFlow::Continue => "continuing".to_string(),
            };
            (StatusCode::OK, body).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn serve() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router()).await?;
    Ok(())
}
