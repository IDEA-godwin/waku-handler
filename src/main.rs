// use std::io::{self, BufRead};
// use std::net::{IpAddr, Ipv4Addr};
// use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};

use waku_bindings::{
    ContentFilter, Encoding, WakuLogLevel, WakuMessage, WakuNodeConfig, WakuPubSubTopic,
    waku_create_content_topic, waku_new, waku_set_event_callback,
};
// use waku_sys::waku_relay_subscribe;

// #[tokio::main]
fn main() {
    dotenvy::dotenv().ok();

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

    println!("boststrap nodes: {:?}", bootstrap_nodes);
    let _topic = WakuPubSubTopic::new();

    let config = WakuNodeConfig {
        port: Some(tcp_port.unwrap()),
        log_level: Some(WakuLogLevel::Warn),
        discv5: Some(true),
        discv5_udp_port: Some(udp_port.unwrap()),
        discv5_bootstrap_nodes: bootstrap_nodes,
        ..Default::default()
    };
    let node = waku_new(Some(config)).expect("should initiate");

    let node = node.start().expect("should start");
    println!("\n===== Node Started =====");
    println!("PeerId: {}", node.peer_id().unwrap());
    thread::sleep(Duration::new(5, 0));

    waku_set_event_callback(|signal| {
        let _event = signal.event();
        println!("Received event");
    });

    let peer_count = node.peer_count().unwrap();
    println!("Initial peer count: {}", peer_count);
    println!(
        "Listening addresses: {:?}",
        node.listen_addresses().unwrap()
    );

    let topic = WakuPubSubTopic::from("/waku/2/m3tering/proto");
    let content_topic = waku_create_content_topic("m3tering", "0.1.0", &topic, Encoding::Proto);

    let _ = node
        .relay_subscribe(&ContentFilter::new(
            Some(topic),
            vec![content_topic.clone()],
        ))
        .unwrap();

    let timestamp = Instant::now().elapsed().as_millis() as usize;
    let _msg = WakuMessage::new(
        "node one was hear".as_bytes(),
        content_topic,
        1,
        timestamp,
        [],
        false,
    );

    println!("Type 'q' to quit, 'p' to check peers...\n");

    // let stdin = io::stdin();
    // let mut handle = stdin.lock();
    // let mut line = String::new();

    loop {
        println!("Start message publishing and peer monitoring loop...");
        let peers = node.peers().unwrap();
        for peer in &peers {
            println!("Peer ID: {}, Is Connected: {:?}", peer.peer_id(), peer.connected());
            println!("is node {}", peer.peer_id().eq(&node.peer_id().unwrap()));
        }

        thread::sleep(Duration::new(30, 0));

        // let inbound = node.
        // let relay_enough_peers = node.relay_enough_peers(Some("/waku/2/m3tering/proto".to_string()));
        // println!("Relay has enough peers: {}", relay_enough_peers.unwrap());

        // let msg_id = node
        //     .relay_publish_message(&msg, None, Some(Duration::new(1000, 0)))
        //     .expect("published message");
        // println!("Published message with id {}", msg_id);

        // line.clear();
        // match handle.read_line(&mut line) {
        //     Ok(_) => {
        //         let cmd = line.trim();
        //         if cmd == "q" {
        //             break;
        //         } else if cmd == "p" {
        //             let peer_count = node.peer_count().unwrap_or(0);
        //             println!("Current peer count: {}", peer_count);
        //         } else if cmd == "l" {
        //             println!(
        //                 "Listening addresses: {:?}",
        //                 node.listen_addresses().unwrap()
        //             );
        //         }
        //     }
        //     Err(e) => {
        //         eprintln!("Error reading input: {}", e);
        //     }
        // }
    }
}
