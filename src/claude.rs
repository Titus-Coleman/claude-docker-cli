use clust::messages::ClaudeModel;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::StreamChunk;
use clust::messages::StreamOption;
use clust::messages::SystemPrompt;
use clust::Client;
use clust::{ApiKey, Client as Claude};
use tokio_stream::StreamExt;

#[tokio::main]
async fn claude_client() -> anyhow::Result<()> {
    let api_key = env::var("ANTHROPIC_API_KEY").unwrap_or_else(|_| "".to_string());
    let claude_client = Claude::from_api_key(ApiKey::new(api_key));
}
