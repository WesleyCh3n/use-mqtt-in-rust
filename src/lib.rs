use paho_mqtt as mqtt;
use std::time::Duration;
use std::{env, thread};

#[derive(Clone)]
pub struct MqttConfig {
    pub uri: String,
    pub user_name: String,
    pub password: String,
    pub client_id: String,
}

pub fn get_mqtt_config() -> MqttConfig {
    MqttConfig {
        uri: env::var("uri").expect("failed to load env uri"),
        user_name: env::var("user_name").expect("failed to load env user_name"),
        password: env::var("password").expect("failed to load env password"),
        client_id: env::var("client_id").expect("failed to load env client_id"),
    }
}

pub fn get_client(cfg: MqttConfig) -> Result<mqtt::Client, mqtt::Error> {
    let opts = mqtt::CreateOptionsBuilder::new()
        .mqtt_version(mqtt::MQTT_VERSION_5)
        .server_uri(cfg.uri)
        .client_id(cfg.client_id)
        .client_id("client1")
        .finalize();

    let connect_opts = mqtt::ConnectOptionsBuilder::new()
        .user_name(cfg.user_name)
        .password(cfg.password)
        .mqtt_version(mqtt::MQTT_VERSION_5)
        .keep_alive_interval(Duration::from_secs(60))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(60))
        .clean_session(true)
        .finalize();

    let client = mqtt::Client::new(opts)?;
    client.connect(connect_opts)?;

    Ok(client)
}

pub fn get_async_client(
    cfg: MqttConfig,
) -> Result<mqtt::AsyncClient, mqtt::Error> {
    let opts = mqtt::CreateOptionsBuilder::new()
        .mqtt_version(mqtt::MQTT_VERSION_5)
        .server_uri(cfg.uri)
        .client_id(cfg.client_id)
        .client_id("client1")
        .finalize();

    let connect_opts = mqtt::ConnectOptionsBuilder::new()
        .user_name(cfg.user_name)
        .password(cfg.password)
        .mqtt_version(mqtt::MQTT_VERSION_5)
        .keep_alive_interval(Duration::from_secs(60))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(60))
        .clean_session(true)
        .finalize();

    let client = mqtt::AsyncClient::new(opts)?;
    client.connect(connect_opts).wait()?;

    Ok(client)
}

pub fn try_reconnect(cli: &mqtt::Client) -> bool {
    println!("Connection lost. Waiting to retry connection");
    for _ in 0..12 {
        thread::sleep(Duration::from_millis(5000));
        if cli.reconnect().is_ok() {
            println!("Successfully reconnected");
            return true;
        }
    }
    println!("Unable to reconnect after several attempts.");
    false
}
