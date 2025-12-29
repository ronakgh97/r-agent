use anyhow::{Context, Result};

pub async fn run_agent(
    task: &str,
    plan: &Option<String>,
    config: &str,
    session: &Option<String>,
    context: &Option<String>,
) -> Result<()> {
    println!("Running agent with the following parameters:");
    println!("Task: {}", task);
    if let Some(p) = plan {
        println!("Plan: {}", p);
    } else {
        println!("Plan: None");
    }

    println!("Config: {}", config);

    if let Some(s) = session {
        println!("Session: {}", s);
    } else {
        println!("Session: New");
    }
    if let Some(ctx) = context {
        println!("Context: {}", ctx);
    } else {
        println!("Context: None");
    }

    unimplemented!();

    Ok(())
}

pub async fn read_stdin() -> Option<String> {
    use tokio::io::{self, AsyncReadExt};

    if atty::is(atty::Stream::Stdin) {
        return None;
    }

    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin
        .read_to_string(&mut buffer)
        .await
        .with_context(|| anyhow::anyhow!("Failed to read from pipe"))
        .ok()?;
    if buffer.trim().is_empty() {
        None
    } else {
        Some(buffer)
    }
}
