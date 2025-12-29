use anyhow::{Result, anyhow};
use my_lib::api::tools_registry::Tool;
use serde_json::Value;
use tokio::process::Command;

/// A tool to list files and directories in the current directory using the `ls` command.
pub struct LsTool;

#[async_trait::async_trait]
impl Tool for LsTool {
    fn name(&self) -> &str {
        "ls_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Runs ls command to list files/dirs in current directory",
                "parameters": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, args: Value) -> Result<String> {
        //TODO: ls with flags support
        let output = Command::new("ls")
            .output()
            .await
            .map_err(|e| anyhow!("Failed to execute ls command: {}", e))?;
        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(result)
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr).to_string();
            Err(anyhow!("ls command failed: {}", error_message))
        }
    }
}

pub struct CatTool;

#[async_trait::async_trait]
impl Tool for CatTool {
    fn name(&self) -> &str {
        "cat_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Prints the contents of a file",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path of the file to read"
                        }
                    },
                    "required": ["path"]
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, args: Value) -> Result<String> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow!("missing 'path'"))?;

        let output = Command::new("cat").arg(path).output().await?;

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(result)
        } else {
            Err(anyhow!(String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }
}
