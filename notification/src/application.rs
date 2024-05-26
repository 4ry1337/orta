use shared::configuration::{MessageBrokerSettings, Settings};
use tracing::info;

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        info!("Building notification service");

        info!("Finished notification service build");

        Ok(Self {
            port,
            server,
            address,
        })
    }

    pub async fn run(self) -> Result<()> {
        info!("Consuming messages");
    }
}

async fn connect_rabbitmq(configuration: MessageBrokerSettings) {}
