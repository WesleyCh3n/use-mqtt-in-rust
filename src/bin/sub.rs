use dotenv::dotenv;
use paho_mqtt as mqtt;

use my_mqtt_lib::{get_mqtt_client, try_reconnect, get_mqtt_config};

fn main() -> Result<(), mqtt::Error> {
    dotenv().ok();
    let cfg = get_mqtt_config();
    let client = get_mqtt_client(cfg)?;
    client.subscribe("cmd/pub", 0)?;
    let rx = client.start_consuming();
    for msg in rx.iter() {
        if let Some(msg) = msg {
            println!("{}", msg.payload_str());
        } else if client.is_connected() || !try_reconnect(&client) {
            break;
        }
    }
    Ok(())
}
