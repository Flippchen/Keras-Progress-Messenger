use reqwest::Client;
use serde::Serialize;


/// Represents the content of a Discord webhook message
#[derive(Debug, Serialize)]
struct DiscordMessage {
    username: String,
    embeds: Vec<DiscordEmbed>,
}

/// Represents an embed within a Discord message
#[derive(Debug, Serialize)]
struct DiscordEmbed {
    title: String,
    description: String,
    color: u32,
}

/// Sends a message to a Discord server using a webhook
pub(crate) async fn send_discord_notification(webhook: &str, payload: &serde_json::Value) {

    // Create a Discord embed with the given payload
    let embed = DiscordEmbed {
        title: "Update Keras Model".to_string(),
        description: payload.to_string(),
        color: 0x000000,
    };
    let discord_message = DiscordMessage {
        username: "Training".to_string(),
        embeds: vec![embed],
    };

    // JSON-encode the message
    let discord_message_json = serde_json::to_string(&discord_message).unwrap();

    // Send request
    let client = Client::new();
    let _ = client
        .post(webhook)
        .header("Content-Type", "application/json")
        .body(discord_message_json)
        .send()
        .await
        .expect("Failed to send request");
}