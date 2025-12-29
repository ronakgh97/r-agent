use crate::core::session::Session;
use my_lib::api::agents::Agent;

#[derive(Debug, Clone)]
pub struct RunnerContext {
    pub task: String,
    pub plan: Option<String>,
    pub config: String,
    pub session: String,
    pub context: Option<String>,
}

impl RunnerContext {
    pub async fn pre_load(agent_config: &Agent, session_data: &Session) {
        unimplemented!();
    }

    pub async fn run(&self, agent_config: &Agent, session_data: &mut Session) {
        unimplemented!();
    }

    pub async fn post_save(agent_config: &Agent, session_data: &Session) {
        unimplemented!();
    }
}
