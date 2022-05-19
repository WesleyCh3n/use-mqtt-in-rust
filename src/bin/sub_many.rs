use dotenv::dotenv;
use paho_mqtt as mqtt;

use my_mqtt_lib::{get_client, get_mqtt_config, try_reconnect};

fn main() -> Result<(), mqtt::Error> {
    dotenv().ok();
    let cfg = get_mqtt_config();
    let cli = get_client(cfg)?;
    cli.subscribe_many(&["test/msg1", "test/msg2"], &[0, 0])?;

    let rx = cli.start_consuming();
    for msg in rx.iter() {
        if let Some(msg) = msg {
            println!("topic: {}", msg.topic());
            println!("payload: {}", msg.payload_str());
        } else if cli.is_connected() || !try_reconnect(&cli) {
            break;
        }
    }

    Ok(())
}
