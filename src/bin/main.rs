use anyhow::Result;
use clap::Parser;
use r_agent::args::{Args, Commands};
use r_agent::cmd::ascii::run_ascii_art;
use r_agent::cmd::init::run_init;
use r_agent::cmd::run::{read_stdin, run_agent};

#[tokio::main]
pub async fn main() -> Result<()> {
    let piped_input = read_stdin().await;
    let cli_args = Args::parse();

    match cli_args.command {
        Some(Commands::Init { fix }) => {
            run_init(fix).await?;
        }
        Some(Commands::Run {
            task,
            plan,
            config,
            session,
        }) => {
            let context = piped_input.clone();
            run_agent(&task, &plan, &config, &session, &context).await?;
        }

        _ => {
            run_ascii_art().await;
        }
    }

    Ok(())
}
