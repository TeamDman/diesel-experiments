use dotenvy::dotenv;
use std::env;
use tokio::sync::mpsc;
use tokio_postgres::{connect, AsyncMessage, NoTls};
use futures::future::poll_fn;

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
    // Load environment variables from the .env file
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the PostgreSQL database
    let (client, mut connection) = connect(&database_url, NoTls).await?;

    // Create an unbounded channel for sending messages
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Spawn a task to handle incoming messages from the connection
    tokio::spawn(async move {
        loop {
            // Poll for the next message from the connection
            let message = poll_fn(|cx| connection.poll_message(cx)).await;
            
            match message {
                Some(Ok(message)) => {
                    // Send the message through the channel
                    if tx.send(message).is_err() {
                        // Receiver has been dropped; exit the loop
                        break;
                    }
                }
                Some(Err(e)) => {
                    eprintln!("Connection error: {}", e);
                    break;
                }
                None => {
                    // Connection has been closed; exit the loop
                    break;
                }
            }
        }
    });

    println!("Listener task spawned.");

    // Execute the LISTEN command to subscribe to notifications
    client
        .batch_execute("LISTEN test_notifications;")
        .await
        .expect("Failed to execute LISTEN command");

    // Continuously receive and handle messages from the channel
    while let Some(message) = rx.recv().await {
        match message {
            AsyncMessage::Notification(notification) => {
                println!(
                    "Received notification on channel '{}': {}",
                    notification.channel(),
                    notification.payload()
                );
            }
            _ => {
                // Handle other types of messages if necessary
                println!("Received non-notification message: {:?}", message);
            }
        }
    }

    Ok(())
}
