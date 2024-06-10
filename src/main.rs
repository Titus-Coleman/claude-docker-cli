use clust::messages::ClaudeModel;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::MessagesResponseBody;
use clust::messages::SystemPrompt;
use clust::{ApiKey, Client as Claude};
use dotenv::dotenv;
use indicatif::ProgressBar;
use sqlformat::{format, FormatOptions, QueryParams};
use std::env;
use std::time::Duration;
mod db;
mod prompt;
mod schema;
mod user_input;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let schema = schema::schema();

    let user_input = user_input::user_input();

    let api_key = env::var("ANTHROPIC_API_KEY").unwrap_or_else(|_| "".to_string());
    let claude_client = Claude::from_api_key(ApiKey::new(api_key));

    let model = ClaudeModel::Claude3Opus20240229;
    let messages = vec![Message::user(user_input)];
    let max_tokens = MaxTokens::new(1024, model)?;
    let system_prompt = SystemPrompt::new(prompt::prompt(schema));
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        ..Default::default()
    };

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));

    pb.set_message("Waiting for response...");

    let raw_response_result: MessagesResponseBody =
        claude_client.create_a_message(request_body).await?;

    pb.finish_with_message("Response received!");

    let formatted_query = format(
        &raw_response_result.content.flatten_into_text().unwrap(),
        &QueryParams::None,
        FormatOptions::default(),
    );

    println!("Formatted Result:\n{}", formatted_query);

    Ok(())
}
