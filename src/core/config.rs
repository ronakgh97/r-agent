use crate::core::tools::get_default_toolset;
use anyhow::{Context, Result};
use my_lib::api::agents::{Agent, AgentBuilder};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;

pub const SYSTEM_PROMPT: &str = "
You are an expert AI coding AGENT with deep knowledge across multiple programming languages, frameworks, and best practices.

Your capabilities:
- Write clean, efficient, and well-documented code
- Debug and fix errors with precision
- Explain complex technical concepts clearly
- Follow language-specific conventions and idioms
- Provide production-ready solutions
- Suggest optimal algorithms and data structures
- Review code for bugs, performance, and security issues

Guidelines:
1. Be concise but thorough in explanations
2. Use proper error handling and edge case coverage
3. Follow the existing code style and patterns
4. Provide working, tested solutions
5. Explain your reasoning when making architectural decisions
6. Prioritize readability and maintainability
7. Suggest improvements proactively
8. When debugging, identify root causes, not just symptoms

When writing code:
- Use descriptive variable and function names
- Add comments for complex logic
- Handle errors gracefully
- Consider performance implications
- Write idiomatic code for the language being used

Always strive for excellence and precision in your responses.";

pub fn default_agents() -> Vec<Agent> {
    vec![
        AgentBuilder::new()
            .model("qwen/qwen3-8b")
            .url("http://localhost:1234/v1")
            .api_key("local")
            .system_prompt(SYSTEM_PROMPT)
            .tool_registry(Arc::new(get_default_toolset()))
            .build()
            .unwrap(),
        AgentBuilder::new()
            .model("deepseek/deepseek-r1-0528-qwen3-8b")
            .url("http://localhost:1124/v1")
            .api_key("YOUR_LUAN_API_KEY")
            .system_prompt(SYSTEM_PROMPT)
            .tool_registry(Arc::new(get_default_toolset()))
            .build()
            .unwrap(),
        AgentBuilder::new()
            .model("qwen/qwen3-coder:free")
            .url("https://openrouter.ai/v1")
            .api_key("YOUR_OPENROUTER_API_KEY")
            .system_prompt(SYSTEM_PROMPT)
            .tool_registry(Arc::new(get_default_toolset()))
            .build()
            .unwrap(),
    ]
}

pub async fn get_agent_configs(config_dir: PathBuf, model: &str) -> Result<Agent> {
    let sanitized_name = model.replace("/", "_").replace(":", "_");
    let file_format = format!("{}.toml", sanitized_name);

    // Get the config in dir
    let file_path = config_dir.join(file_format);
    let config_data = tokio::fs::read_to_string(&file_path).await?;
    let agent_builder = AgentBuilder::load_from_toml(&config_data)?;
    let agent = agent_builder.build()?;
    Ok(agent)
}

pub async fn save_default_agent_configs(agent: &Agent, path: PathBuf) -> Result<()> {
    let agent_str = AgentBuilder::convert_to_builder(agent).to_toml_string()?;

    // Save the toml - sanitize filename by replacing invalid characters
    let sanitized_name = agent.model.replace("/", "_").replace(":", "_");
    let file_name = format!("{}.toml", sanitized_name);
    let file_path = path.join(file_name);
    fs::write(file_path, agent_str).await?;

    Ok(())
}

pub async fn load_config(agent_config: String) -> Result<String> {
    let config_dir = get_default_config_path()
        .with_context(|| anyhow::anyhow!("Failed to get default config path"))?;
    let config_file_name = format!("{}.toml", agent_config);
    let config_path = config_dir.join(config_file_name);
    let config_body = tokio::fs::read_to_string(&config_path)
        .await
        .with_context(|| anyhow::anyhow!("Failed to read config file"))?;
    Ok(config_body)
}

pub fn get_default_config_path() -> Result<PathBuf> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let config_path = home_dir.join(".config").join("r_agent").join("config");
    Ok(config_path)
}

pub async fn create_config_dir() -> Result<PathBuf> {
    let config_path = get_default_config_path()?;
    fs::create_dir_all(&config_path).await?;

    Ok(config_path)
}
