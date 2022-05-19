use dotenv::dotenv;
use paho_mqtt as mqtt;

use my_mqtt_lib::{get_client, get_mqtt_config};

fn main() -> Result<(), mqtt::Error> {
    dotenv().ok();
    let cfg = get_mqtt_config();
    let client = get_client(cfg)?;
    let msg = mqtt::MessageBuilder::new()
        .topic("cmd/return")
        .payload("Test Msg")
        .qos(0)
        .finalize();
    client.publish(msg)?;
    Ok(())
}
