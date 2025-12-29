use anyhow::Result;
use my_lib::api::agents::{Agent, AgentBuilder};
use std::path::PathBuf;
use tokio::fs;

pub fn default_agents() -> Vec<Agent> {
    vec![
        AgentBuilder::new()
            .model("qwen/qwen3-8b")
            .url("http://localhost:1234/v1")
            .api_key("local")
            .system_prompt("")
            .build()
            .unwrap(),
        AgentBuilder::new()
            .model("deepseek/deepseek-r1-0528-qwen3-8b")
            .url("http://localhost:1124/v1")
            .api_key("YOUR_LUAN_API_KEY")
            .system_prompt("")
            .build()
            .unwrap(),
        AgentBuilder::new()
            .model("qwen/qwen3-coder:free")
            .url("https://openrouter.ai/v1")
            .api_key("YOUR_OPENROUTER_API_KEY")
            .system_prompt("")
            .build()
            .unwrap(),
    ]
}

pub async fn get_agent_configs(config_dir: PathBuf, model: &str) -> Result<Agent> {
    let file_format = format!("{}.toml", model.replace("/", "_"));

    // Get the config in dir
    let file_path = config_dir.join(file_format);
    let config_data = tokio::fs::read_to_string(&file_path).await?;
    let agent_builder = AgentBuilder::load_from_toml(&config_data)?;
    let agent = agent_builder.build()?;
    Ok(agent)
}

pub async fn save_default_agent_configs(agent: &Agent, path: PathBuf) -> Result<()> {
    let agent_str = AgentBuilder::convert_to_builder(agent).to_toml_string()?;

    // Save the toml
    let file_name = format!("{}.toml", agent.model.replace("/", "_"));
    let file_path = path.join(file_name);
    fs::write(file_path, agent_str).await?;

    Ok(())
}

pub async fn create_config_dir() -> Result<PathBuf> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let config_path = home_dir.join(".config").join(".r_agent").join("config");

    fs::create_dir_all(&config_path).await?;

    Ok(config_path)
}
