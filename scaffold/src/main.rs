//! Factor 11 — one binary, many triggers: CLI and webhook.
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("webhook") => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(my_project::triggers::webhook::serve())
        }
        _ => my_project::triggers::cli::run_main(&args),
    }
}
