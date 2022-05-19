use dotenv::dotenv;
use paho_mqtt as mqtt;
use sysinfo::{ProcessExt, System, SystemExt};

use my_mqtt_lib::{get_client, get_mqtt_config, try_reconnect};

use std::process::Command;

fn main() -> Result<(), mqtt::Error> {
    dotenv().ok();
    let cfg = get_mqtt_config();
    let client = get_client(cfg)?;
    client.subscribe("cmd/pub", 0)?;

    let cmds = ["python3", "long_process.py"];

    let rx = client.start_consuming();
    for msg in rx.iter() {
        if let Some(msg) = msg {
            if msg.payload_str() == "start" {
                println!("Start program!");
                let child = Command::new(cmds[0])
                    .args(&cmds[1..])
                    .spawn()
                    .expect("failed to execute process");

                let msg = mqtt::MessageBuilder::new()
                    .topic("cmd/return")
                    .payload(child.id().to_string())
                    .qos(0)
                    .finalize();
                client.publish(msg)?;
            } else if msg.payload_str() == "kill" {
                let s = System::new_all();
                for process in s.processes_by_name(cmds[0]) {
                    if process.cmd() == cmds {
                        println!("Kill program!");
                        process.kill();
                    }
                }
            }
        } else if client.is_connected() || !try_reconnect(&client) {
            break;
        }
    }

    Ok(())
}
