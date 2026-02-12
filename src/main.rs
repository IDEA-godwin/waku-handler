use std::io::{self, BufRead};
use std::str::FromStr;
use std::time::{Duration, Instant};

use waku_bindings::{
    Encoding, Multiaddr, WakuLogLevel, WakuMessage, WakuNodeConfig, WakuPubSubTopic,
    waku_create_content_topic, waku_new, waku_set_event_callback,
};

// #[tokio::main]
fn main() {
    dotenvy::dotenv().ok();
    let args: Vec<String> = std::env::args().collect();

    let peer_address: Vec<String> = args.iter().skip(1).cloned().collect();

    let tcp_port = std::env::var("WAKU_HANDLER_PORT")
        .unwrap_or_else(|_| "60100".to_string())
        .parse::<usize>();

    let udp_port = std::env::var("WAKU_HANDLER_UDP_PORT")
        .unwrap_or_else(|_| "9000".to_string())
        .parse::<u16>();

    let bootstrap_nodes: Vec<String> = std::env::var("WAKU_HANDLER_BOOTSTRAP_NODES")
        .unwrap_or_else(|_| "".to_string())
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect();

    let _topic = WakuPubSubTopic::new();

    let config = WakuNodeConfig {
        port: Some(tcp_port.unwrap()),
        log_level: Some(WakuLogLevel::Info),
        relay: Some(true),
        // relay_topics: vec!["waku/2/m3tering".to_string()],
        discv5: Some(true),
        discv5_udp_port: Some(udp_port.unwrap()),
        discv5_bootstrap_nodes: bootstrap_nodes,
      
        keep_alive_interval: Some(5),
        ..Default::default()
    };
    let node = waku_new(Some(config)).expect("should initiate");

    // let second_node = waku_new(Some(config))
    //    .expect("should initiate");

    waku_set_event_callback(|signal| {
        let _event = signal.event();
        println!("Received event");
    });

    let node = node.start().expect("should start");

    println!("\n===== Node Started =====");
    println!("PeerId: {}", node.peer_id().unwrap());
    if peer_address.is_empty() {
        println!("No peer address provided. Running in standalone mode.");
    } else {
        for peer in &peer_address {
            println!("Attempting to connect to peer at: {}", peer);

            let address = Multiaddr::from_str(peer.as_str()).expect("should parse multiaddr");
            let _ = node
                .add_peer(&address, waku_bindings::ProtocolId::Filter)
                .expect("should add peer");
        }
    }

    let peer_count = node.peer_count().unwrap();
    println!("Initial peer count: {}", peer_count);

    let topic = "/waku/2/m3tering";
    let timestamp = Instant::now().elapsed().as_millis() as usize;
    let content_topic = waku_create_content_topic("m3tering", "0.1.0", topic, Encoding::Rfc26);
    let _msg = WakuMessage::new(
        "node one was hear".as_bytes(),
        content_topic,
        1,
        timestamp,
        [],
        false,
    );
    // let msg_id = node
    //    .relay_publish_message(&msg, None, Some(Duration::new(1000, 0)))
    //    .expect("published message");
    // println!("Published message with id {}", msg_id);

    println!("Type 'q' to quit, 'p' to check peers...\n");

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();

    loop {
        line.clear();
        match handle.read_line(&mut line) {
            Ok(_) => {
                let cmd = line.trim();
                if cmd == "q" {
                    break;
                } else if cmd == "p" {
                    let peer_count = node.peer_count().unwrap_or(0);
                    println!("Current peer count: {}", peer_count);
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
            }
        }
    }

    node.stop().unwrap();
}
