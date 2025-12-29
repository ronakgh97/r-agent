use crate::core::config::{create_config_dir, default_agents, save_default_agent_configs};
use crate::core::data::create_data_source;
use crate::core::session::create_session_dir;
use anyhow::Result;

pub async fn run_init(fix: bool) -> Result<()> {
    if fix {
        unimplemented!();
    }

    let config_path = create_config_dir().await?;
    let agents = default_agents();

    // Save default agent configs
    for agent in agents.iter() {
        save_default_agent_configs(agent, config_path.clone()).await?;
    }

    let _ = create_session_dir().await?;

    let _ = create_data_source().await?;

    Ok(())
}
