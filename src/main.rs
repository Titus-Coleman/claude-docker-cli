use clust::messages::ClaudeModel;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::SystemPrompt;
use clust::{ApiKey, Client as Claude};
use dotenv::dotenv;
use std::env;
mod db;
mod user_input;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let user_input = user_input::user_input();
    // let db_client = db::establish_db_connection();

    let api_key = env::var("ANTHROPIC_API_KEY").unwrap_or_else(|_| "".to_string());
    let claude_client = Claude::from_api_key(ApiKey::new(api_key));

    let model = ClaudeModel::Claude3Opus20240229;
    let messages = vec![Message::user(user_input)];
    let max_tokens = MaxTokens::new(1024, model)?;
    let system_prompt = SystemPrompt::new("Hello");
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        ..Default::default()
    };

    let response = claude_client.create_a_message(request_body).await?;

    println!("Result: \n{}", response);

    Ok(())
}
