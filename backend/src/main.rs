use backend::{
    application::Application, configuration::CONFIG, utils::fingerprint::generate_fingerprint,
};

//TODO: add multithreading
//TODO: add rate limiter? mb middleware

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    tracing::subscriber::set_global_default(subscriber)?;

    let application = Application::build(CONFIG.clone()).await?;

    application.run().await?;

    Ok(())
}
