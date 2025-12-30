use crate::core::session::MappedMessage;
use crate::core::session::Session;
use crate::core::tools::get_default_toolset;
use anyhow::Result;
use my_lib::api::agents::{Agent, AgentBuilder, prompt_with_tools_stream};
use my_lib::api::dtos::Message;
use my_lib::api::request::log_typewriter_effect;
use std::sync::Arc;

#[derive(Clone)]
pub struct RunnerContext {
    //TODO: Implement Plan Handling
    pub agent_config: Agent,
    pub session: Option<Session>,
    pub context: Option<String>,
}

impl RunnerContext {
    /// Preload context and tools before running the agent, because tools cant be serialized and be saved in json/toml
    pub async fn pre_load(
        agent_config: &str,
        session_data: &Option<Session>,
        context: &Option<String>,
    ) -> Result<Self> {
        let agent_builder: AgentBuilder = toml::from_str(&agent_config)?;
        let agent_config = agent_builder
            .tool_registry(Arc::new(get_default_toolset()))
            .build()?;

        Ok(Self {
            agent_config: agent_config.clone(),
            session: session_data.clone(),
            context: context.clone(),
        })
    }

    /// Run the agent with the given task and agent configuration, but without session.
    pub async fn run(&self, task: String) -> Result<()> {
        let mut user_prompt = task.clone();

        // Add context to history if available
        if let Some(ref ctx) = self.context {
            user_prompt = format!("Context: {}\n\n{}", ctx, user_prompt);
        }

        let history: Vec<Message> = vec![Message {
            role: my_lib::api::dtos::Role::USER,
            content: Some(user_prompt),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        }];

        let stream = prompt_with_tools_stream(self.agent_config.clone(), history.clone()).await?;

        let _ = log_typewriter_effect(120, stream).await?;

        Ok(())
    }

    /// Run the agent session with the given task and update the session data.
    pub async fn run_session(&self, task: String, session_data: &mut Session) -> Result<()> {
        let mut user_prompt = task.clone();

        // Add context to history if available
        if let Some(ref ctx) = self.context {
            user_prompt = format!("Context: {}\n\n User: {}", ctx, user_prompt);
        }

        let mut history = session_data.messages.clone();
        history.push(Message {
            role: my_lib::api::dtos::Role::USER,
            content: Some(user_prompt),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        });

        let stream = prompt_with_tools_stream(self.agent_config.clone(), history.clone()).await?;

        let stream_to_str = log_typewriter_effect(120, stream).await?;
        let agent_message = Message {
            role: my_lib::api::dtos::Role::ASSISTANT,
            content: Some(stream_to_str),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        };
        history.push(agent_message);

        // Update session messages
        session_data.messages = history;
        session_data.last_model_used = self.agent_config.model.clone();
        session_data.save_to_disk().await?;

        Ok(())
    }
}

#[allow(unused)]
pub fn map_message_to(message: &Message) -> MappedMessage {
    match message.role {
        my_lib::api::dtos::Role::USER => {
            if let Some(ref content) = message.content {
                MappedMessage::User(content.clone())
            } else {
                // Fallback for missing content
                MappedMessage::User(String::new())
            }
        }
        my_lib::api::dtos::Role::ASSISTANT => {
            if let Some(ref content) = message.content {
                MappedMessage::Agent(content.clone())
            } else {
                MappedMessage::Agent(String::new())
            }
        }
        _ => {
            // Fallback for unsupported roles
            MappedMessage::User(String::new())
        }
    }
}

#[allow(unused)]
pub fn map_message_from(message: &MappedMessage) -> Message {
    match message {
        MappedMessage::User(content) => Message {
            role: my_lib::api::dtos::Role::USER,
            content: Some(content.clone()),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        },
        MappedMessage::Agent(content) => Message {
            role: my_lib::api::dtos::Role::ASSISTANT,
            content: Some(content.clone()),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        },
    }
}
