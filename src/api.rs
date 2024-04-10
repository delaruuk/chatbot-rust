use leptos::*;
use crate::model::conversation::Conversation;
#[server(Converse "/api")]
pub async fn converse(cs: Scope, prompt: conversation) -> Result<String, ServerFnError> {
    use llm::models::Llama;
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_web::web::ConnectionInfo;
}