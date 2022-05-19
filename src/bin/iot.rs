use chrono;
use dotenv::dotenv;
use paho_mqtt::{Message, MessageBuilder};
use std::{env, process::Command};
use sysinfo::{ProcessExt, System, SystemExt};

use my_mqtt_lib::{get_client, get_mqtt_config, try_reconnect};

fn control_task(payload: String, node: String) -> Message {
    let output = Command::new("sh")
        .arg("-c")
        .arg(payload)
        .output()
        .expect("failed to execute process");
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let result = format!(
        "{} Node: {} Status: {} Output:\n{}",
        now,
        node,
        output.status.code().unwrap(),
        if output.status.success() {
            std::str::from_utf8(&output.stdout).unwrap()
        } else {
            std::str::from_utf8(&output.stderr).unwrap()
        },
    );
    MessageBuilder::new()
        .topic("control/cmd/output")
        .payload(result)
        .qos(0)
        .finalize()
}

fn program_task(
    topic: String,
    payload: String,
    cmds: Vec<String>,
    node: String,
) -> Message {
    let mut result = String::new();
    if payload == "start" {
        let child = Command::new(&cmds[0])
            .args(&cmds[1..])
            .spawn()
            .expect("failed to execute process");

        result = format!("Node: {} PID: {}", node, child.id());
    } else if payload == "kill" {
        let s = System::new_all();
        for process in s.processes_by_name(&cmds[0]) {
            if process.cmd() == cmds {
                process.kill();
                result =
                    format!("Killed: Node: {} PID: {}", node, process.pid());
            }
        }
    } else {
        result = format!("{} is not valid payload", payload);
    }
    MessageBuilder::new()
        .topic(topic.clone() + "/output")
        .payload(result)
        .qos(0)
        .finalize()
}

fn main() {
    dotenv().ok();
    let node = env::var("node").expect("failed to load env node");

    let cfg = get_mqtt_config();
    let cli = get_client(cfg).unwrap();

    let topics = vec![
        format!("control/{}/cmd", node),
        "control/cmd".into(),
        "prog/stream".into(),
    ];
    cli.subscribe_many(&topics, &[0, 0, 0]).unwrap();

    let rx = cli.start_consuming();
    for msg in rx.iter() {
        if let Some(msg) = msg {
            if msg.topic() == topics[0] || msg.topic() == topics[1] {
                cli.publish(control_task(
                    msg.payload_str().into(),
                    node.clone(),
                ))
                .unwrap();
            } else if msg.topic() == topics[2] {
                cli.publish(program_task(
                    msg.topic().into(),
                    msg.payload_str().into(),
                    vec!["python".into(), "stream.py".into()],
                    node.clone(),
                ))
                .unwrap();
            }
        } else if cli.is_connected() || !try_reconnect(&cli) {
            break;
        }
    }
}
