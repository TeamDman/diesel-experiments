use dotenvy::dotenv;
use std::env;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use tokio_postgres::connect;
use tokio_postgres::tls::NoTlsStream;
use tokio_postgres::AsyncMessage;
use tokio_postgres::NoTls;
use tokio_postgres::Socket;
use tokio_stream::Stream;
use tokio_stream::StreamExt;

struct ConnectionStream<'a> {
    connection: &'a mut tokio_postgres::Connection<Socket, NoTlsStream>,
}

impl<'a> Stream for ConnectionStream<'a> {
    type Item = Result<AsyncMessage, tokio_postgres::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.connection.poll_message(cx) {
            Poll::Ready(Some(Ok(message))) => Poll::Ready(Some(Ok(message))),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
    // Load environment variables from the .env file
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database
    let (client, mut connection) = connect(&database_url, NoTls).await?;

    // Listen to the 'events' channel
    client.batch_execute("LISTEN test_event").await?;

    println!("Listening for events...");

    // Create a stream from the connection
    let mut stream = ConnectionStream {
        connection: &mut connection,
    };

    // Process messages from the stream
    while let Some(message) = stream.next().await {
        match message? {
            AsyncMessage::Notification(notification) => {
                println!("Received notification: {}", notification.payload());
            }
            _ => {} // Handle other message types if necessary
        }
    }

    Ok(())
}
