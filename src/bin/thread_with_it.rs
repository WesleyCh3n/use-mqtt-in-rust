use dotenv::dotenv;
use paho_mqtt::AsyncClient;
use std::thread;

use my_mqtt_lib::{get_async_client, get_mqtt_config};

fn mqtt_task(topic: String, client: AsyncClient) {
    println!("Sub to {}", topic);
    client.subscribe(topic, 0).wait().unwrap();

    let rx = client.start_consuming();
    for msg in rx.iter() {
        if let Some(msg) = msg {
            println!("{}", msg);
        } else {
            println!("Lost connection. Attempting reconnect.");
            while let Err(err) = client.reconnect().wait() {
                println!("Error reconnecting: {}", err);
            }
        }
    }
}

fn main() {
    dotenv().ok();
    let cfg = get_mqtt_config();
    let cli = get_async_client(cfg).unwrap();
    let cli2 = cli.clone();
    let cli3 = cli.clone();

    let handle1 = thread::spawn(move || {
        mqtt_task("test/msg1".to_string(), cli);
    });
    let handle2 = thread::spawn(move || {
        mqtt_task("test/msg2".to_string(), cli2);
    });
    let handle3 = thread::spawn(move || {
        mqtt_task("test/msg3".to_string(), cli3);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}
