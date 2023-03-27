use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

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

/// Processes the payload of a webhook message
fn process_payload(payload: &Value) -> String {
    let accuracy = payload["accuracy"].as_f64().unwrap_or(0.0);
    let loss = payload["loss"].as_f64().unwrap_or(0.0);
    let val_accuracy = payload["val_accuracy"].as_f64().unwrap_or(0.0);
    let val_loss = payload["val_loss"].as_f64().unwrap_or(0.0);

    format!(
        "Accuracy: {:.6}\nLoss: {:.6}\nValidation Accuracy: {:.6}\nValidation Loss: {:.6}",
        accuracy,  loss, val_accuracy, val_loss
    )
}

/// Sends a message to a Discord server using a webhook
pub(crate) async fn send_discord_notification(webhook: &str, payload: &Value) {

    // Extract the epoch from payload
    let epoch = payload["epoch"].as_i64().unwrap_or(0);
    // Process the payload
    let message: String = process_payload(payload);

    // Create a Discord embed with the given payload
    let embed = DiscordEmbed {
        title: format!("Epoch: {}", epoch).to_string(),
        description: message.to_string(),
        color: 0x000000,
    };
    let discord_message = DiscordMessage {
        username: "Training Updates".to_string(),
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