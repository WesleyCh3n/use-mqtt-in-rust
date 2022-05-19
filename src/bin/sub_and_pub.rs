use dotenv::dotenv;
use paho_mqtt as mqtt;

use my_mqtt_lib::{get_client, get_mqtt_config, try_reconnect};

use std::process::Command;

fn main() -> Result<(), mqtt::Error> {
    dotenv().ok();
    let cfg = get_mqtt_config();
    let client = get_client(cfg)?;
    client.subscribe("cmd/pub", 0)?;

    let rx = client.start_consuming();
    for msg in rx.iter() {
        if let Some(msg) = msg {
            let cmd = msg.payload_str().to_string();

            let mut parts = cmd.trim().split_whitespace();
            let cmd = parts.next().unwrap();
            let args = parts;
            let child = Command::new(cmd)
                .args(args)
                .spawn()
                .expect("failed to execute process");
            let msg = mqtt::MessageBuilder::new()
                .topic("cmd/return")
                .payload(child.id().to_string())
                .qos(0)
                .finalize();
            client.publish(msg)?;
        } else if client.is_connected() || !try_reconnect(&client) {
            break;
        }
    }

    Ok(())
}
