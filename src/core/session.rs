use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub name: String,
    pub model_used: String,
    pub path: PathBuf,
    pub messages: Vec<Message>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message {
    User(String),
    Agent(String),
}
impl Session {
    pub fn new(name: &str, model_used: &str, path: PathBuf) -> Self {
        Session {
            name: name.to_string(),
            model_used: model_used.to_string(),
            path,
            messages: Vec::new(),
        }
    }

    pub async fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub async fn save(&self) -> Result<()> {
        let session_str = serde_json::to_string(&self)?;
        tokio::fs::write(&self.path, session_str).await?;
        Ok(())
    }
}

pub async fn load_session(path: PathBuf) -> Result<Session> {
    let session_data = tokio::fs::read_to_string(&path).await?;
    let session: Session = serde_json::from_str(&session_data)?;
    Ok(session)
}

pub async fn create_session_dir() -> Result<PathBuf> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let session_path = home_dir.join(".config").join(".r_agent").join("sessions");

    fs::create_dir_all(&session_path).await?;

    Ok(session_path)
}
