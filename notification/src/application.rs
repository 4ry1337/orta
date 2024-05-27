use std::str;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicAckArguments, BasicConsumeArguments, BasicQosArguments, Channel,
        ExchangeDeclareArguments, QueueBindArguments, QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
};
use lettre::AsyncTransport;
use secrecy::ExposeSecret;
use shared::{
    configuration::{MessageBrokerSettings, Settings},
    utils::message::VerificationMessage,
};
use tokio::{
    sync::Notify,
    time::{sleep, Duration},
};
use tracing::{debug, error, info, warn};

use crate::services::mail::{verification_message, MailService};

pub struct Application {
    connection: Connection,
    channel: Channel,
    mail_service: MailService,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        info!("Building notification service");

        let mut connection = connect_rabbitmq(&configuration.message_broker).await;

        let mut channel = channel_rabbitmq(&connection).await;

        bind_queue_to_exchange(
            &mut connection,
            &mut channel,
            &configuration.message_broker,
            "orta",
            "notification",
            "orta.notification",
        )
        .await;

        let mail_service = MailService::build(&configuration.mail).expect("Mail Service Error");

        info!("Finished notification service build");

        Ok(Self {
            mail_service,
            connection,
            channel,
        })
    }

    pub async fn run(self) {
        let consumer_args = BasicConsumeArguments::new("notification", "");
        info!("Consuming messages");
        tokio::spawn(async move {
            match self.channel.basic_consume_rx(consumer_args.clone()).await {
                Ok((_, mut message_rx)) => {
                    while let Some(message) = message_rx.recv().await {
                        match message.basic_properties {
                            Some(basic_properties) => match basic_properties.message_type() {
                                Some(message_type) => match message.content {
                                    Some(payload) => match str::from_utf8(&payload) {
                                        Ok(paylaod_str) => {
                                            if message_type == "orta.notification.verification" {
                                                match serde_json::from_str::<VerificationMessage>(paylaod_str) {
                                                    Ok(verification_message_paylaod) => match verification_message(
                                                        &verification_message_paylaod.email,
                                                        &verification_message_paylaod.verification_link
                                                    ) {
                                                        Ok(verification_message) => match self.mail_service.mailer.send(verification_message).await {
                                                            Ok(_) => {
                                                                self.channel.basic_ack(BasicAckArguments::new(message.deliver.unwrap().delivery_tag(),false,)).await.unwrap()
                                                            },
                                                            Err(err) => error!("Error sending message: {:?}", err),
                                                        },
                                                        Err(err) => error!("Unable to create verification message: {:?}", err)
                                                    },
                                                    Err(err) => error!("Unable to parse payload: {:?}", err)
                                                }
                                            }
                                            warn!("Unknown message type")
                                        }
                                        Err(err) => {
                                            error!("Unable to parse payload string {:?}", err)
                                        }
                                    },
                                    None => error!("No message content"),
                                },
                                None => error!("No message type"),
                            },
                            None => error!("No basic properties"),
                        }
                    }
                }
                Err(err) => error!("Error consuming message from rabbit: {}", err),
            }
        });

        let guard = Notify::new();
        guard.notified().await;
    }
}

pub async fn connect_rabbitmq(configuration: &MessageBrokerSettings) -> Connection {
    info!("Connecting to rabbitmq");

    let args = OpenConnectionArguments::new(
        &configuration.hostname,
        configuration.port,
        &configuration.username,
        configuration.password.expose_secret(),
    )
    .finish();

    let mut res = Connection::open(&args).await;

    while res.is_err() {
        debug!("trying to connect after error");
        sleep(Duration::from_millis(2000)).await;
        res = Connection::open(&args).await;
    }

    let connection = res.unwrap();

    info!("Connected to rabbitmq at {:?}", configuration);

    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    connection
}

pub async fn channel_rabbitmq(connection: &Connection) -> Channel {
    let channel = connection.open_channel(None).await.unwrap();

    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    channel
}

pub async fn bind_queue_to_exchange(
    connection: &mut Connection,
    channel: &mut Channel,
    configuration: &MessageBrokerSettings,
    exchange: &str,
    queue: &str,
    routing_key: &str,
) {
    if !connection.is_open() {
        warn!("Connection not open");

        *connection = connect_rabbitmq(configuration).await;

        *channel = channel_rabbitmq(connection).await;

        debug!("New connection: {}", connection);
    }
    // Declaring the exchange on startup
    channel
        .exchange_declare(ExchangeDeclareArguments::new(exchange, "direct"))
        .await
        .unwrap();
    // Setting up basic quality-of-service parameters for the channel to enable streaming queue
    match channel
        .basic_qos(BasicQosArguments {
            prefetch_count: 10000,
            prefetch_size: 0,
            global: false,
        })
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error!("An error occurred while setting up the channel:{}", e)
        }
    }

    match channel
        .queue_declare(
            QueueDeclareArguments::default()
                .queue(queue.to_owned())
                .durable(true)
                // .arguments(args)
                .finish(),
        )
        .await
    {
        Ok(queue_option) => {
            match queue_option {
                Some((queue, _, _)) => {
                    //check if the channel is open, if not then open it
                    if !channel.is_open() {
                        warn!(
                            "Channel is not open, does exchange {:?} exist on rabbitMQ?",
                            exchange
                        );
                        *channel = channel_rabbitmq(connection).await;
                    }

                    // bind the queue to the exchange using this channel
                    channel
                        .queue_bind(QueueBindArguments::new(&queue, exchange, routing_key))
                        .await
                        .unwrap();
                }
                None => {}
            }
        }
        Err(err) => {
            error!("An error occurred while setting up the queue: {:?}", err)
        }
    }
}
