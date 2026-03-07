// use std::io::{self, BufRead};
// use std::net::{IpAddr, Ipv4Addr};
// use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};

use waku_bindings::{
    ContentFilter, Encoding, Event, SecretKey, WakuLogLevel, WakuMessage, WakuNodeConfig, WakuPeerData, WakuPubSubTopic, waku_create_content_topic, waku_new, waku_set_event_callback
};
// use waku_sys::waku_relay_subscribe;

// #[tokio::main]
fn main() {
    dotenvy::dotenv().ok();

    let node_key = std::env::var("WAKU_HANDLER_NODE_KEY")
        .map(|a| hex::decode(a).unwrap())
        .map(|a| SecretKey::from_slice(&a).unwrap())
        .ok();

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
        node_key,
        port: Some(tcp_port.unwrap()),
        log_level: Some(WakuLogLevel::Info),
        // relay: Some(true),
        // min_peers_to_publish: Some(1),
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
        let event = signal.event();
        println!("===== Received Event =====");
        match event {
            Event::WakuMessage(msg) => {
                println!("Received message with content topic: {}", msg.waku_message().content_topic());
                println!("Message payload: {:?}", msg.waku_message().payload());
            },
            Event::Unrecognized(msg) => println!("Received unrecognized event: {}", msg),
            _ =>  println!("Received other event")
        };
        println!("==========================");
    });

    let peer_count = node.peer_count().unwrap();
    println!("Initial peer count: {}", peer_count);
    println!(
        "Listening addresses: {:?}",
        node.listen_addresses().unwrap()
    );

    let topic = WakuPubSubTopic::from("/waku/2/default-waku/proto");
    let content_topic = waku_create_content_topic("m3tering", "1", "data-stream", Encoding::Proto);

    let _ = node
        .relay_subscribe(&ContentFilter::new(
            Some(topic.clone()),
            vec![content_topic.clone()],
        ))
        .unwrap();

    // let mut counter = 0;

    loop {
        println!("Start message publishing and peer monitoring loop...");
        // let node_info =
        println!("peer count: {}", node.peer_count().unwrap());
        let peers = node.peers().unwrap();
        for peer in &peers
            .iter()
            .filter(|a| a.connected())
            .collect::<Vec<&WakuPeerData>>()
        {
            println!(
                "Peer ID: {}, \n Protocol: {:?}",
                peer.peer_id(),
                peer.protocols()
            );
        }
        // println!("Relay topics: {:?}", node.relay_topics().unwrap());
        // let relay_enough_peers = node.relay_enough_peers(Some(topic.clone())).unwrap();
        // println!("Relay has enough peers: {}", relay_enough_peers);
        // thread::sleep(Duration::new(30, 0));

        // // let inbound = node.
        // if relay_enough_peers {
        // println!("================ pushing message =================================");
        //     let timestamp = Instant::now().elapsed().as_millis() as usize;
        //     let msg = WakuMessage::new(
        //         format!("node one was hear {}", counter).as_bytes(),
        //         content_topic.clone(),
        //         1,
        //         timestamp,
        //         [],
        //         false,
        //     );

        //     let msg_id = node
        //         .lightpush_publish(&msg, Some(topic.clone()), String::from("16Uiu2HAmU88qxMSeTqRnvjoZ5fkMFQ27rFQCGjh4NRiC7g4MM6UK"), Some(Duration::new(1000, 0)))
        //         .expect("published message");
        //     println!("Published message with id {}", msg_id);
        //     counter += 1;
        // }
    }
}
