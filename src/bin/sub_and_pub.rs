use dotenv::dotenv;
use paho_mqtt as mqtt;

use my_mqtt_lib::{get_mqtt_client, get_mqtt_config, try_reconnect};

use std::process::Command;

fn main() -> Result<(), mqtt::Error> {
    dotenv().ok();
    let cfg = get_mqtt_config();
    let client = get_mqtt_client(cfg)?;
    client.subscribe("cmd/pub", 0)?;

    let rx = client.start_consuming();
    for msg in rx.iter() {
        if let Some(msg) = msg {
            println!("{}", msg.payload_str());

            let cmd = msg.payload_str().to_string();
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect("failed to execute process ");
            let msg = mqtt::MessageBuilder::new()
                .topic("cmd/return")
                .payload(output.stdout)
                .qos(0)
                .finalize();
            client.publish(msg)?;
        } else if client.is_connected() || !try_reconnect(&client) {
            break;
        }
    }

    Ok(())
}
