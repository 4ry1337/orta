use std::str;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicConsumeArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
};
use secrecy::ExposeSecret;
use shared::configuration::CONFIG;
use tokio::sync::Notify;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<tokio::io::Error>> {
    // construct a subscriber that prints formatted traces to stdout
    // Start configuring a `fmt` subscriber
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(true)
        // Build the subscriber
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Building notification service");

    let message_broker = &CONFIG.message_broker;

    let args = OpenConnectionArguments::new(
        &message_broker.hostname,
        message_broker.port,
        &message_broker.username,
        message_broker.password.expose_secret(),
    )
    .finish();

    let connection = Connection::open(&args).await.unwrap();

    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    // open a channel on the connection
    let channel = connection.open_channel(None).await.unwrap();

    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    // declare a durable queue
    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::durable_client_named(
            "email-verification",
        ))
        .await
        .unwrap()
        .unwrap();

    info!(queue_name);

    let consumer_args = BasicConsumeArguments::new(&queue_name, "notification");

    let (_ctag, mut rx) = channel.basic_consume_rx(consumer_args).await.unwrap();

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Some(payload) = msg.content {
                println!(" [x] Received {:?}", str::from_utf8(&payload).unwrap());
            }
        }
    });

    println!(" [*] Waiting for messages. To exit press CTRL+C");

    let guard = Notify::new();
    guard.notified().await;

    Ok(())
}
