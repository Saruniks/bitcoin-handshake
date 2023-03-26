use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

static VERSION_MESSAGE: &[u8] = &[
    // header
    249, 190, 180, 217, // magic
    118, 101, 114, 115, 105, 111, 110, 0, 0, 0, 0, 0, // "version" command
    86, 0, 0, 0, // payload len
    94, 38, 138, 233, // checksum
    // payload
    113, 17, 1, 0, // protocol version
    0, 0, 0, 0, 0, 0, 0, 0, // services
    0, 0, 0, 0, 0, 0, 0, 0, // time
    // 36, 99, 31, 100, 0, 0, 0, 0, // time
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 134, 195, 185, 52, 32, 141, // recipient address info
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0, // sender address info
    0, 0, 0, 0, 0, 0, 0, 0, // Node ID,
    // 131, 67, 161, 229, 100, 242, 249, 181, // Node ID,
    0, // "" sub-version string, 0 bytes long
    0, // relay
    0, 0, 0, 0, // last block sending node
];

static VERACK_MESSAGE: &[u8] = &[
    // header
    249, 190, 180, 217, // magic
    18, 101, 114, 97, 99, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // payload len
    93, 246, 224, 226, // checksum
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = "134.195.185.52:8333".parse().unwrap();

    let mut stream = TcpStream::connect(address).await?;

    let _ = stream.write_all(VERSION_MESSAGE).await;
    read_version_message(&mut stream).await?;

    let _ = stream.write_all(VERACK_MESSAGE).await;
    read_verack_message(&mut stream).await?;
    Ok(())
}

async fn read_version_message(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut magic = vec![0; 4];
    stream.read_exact(&mut magic).await?;

    println!("{:?}", magic);

    let mut cmd = vec![0; 12];
    stream.read_exact(&mut cmd).await?;

    println!("{:?}", cmd);

    let mut len = vec![0; 4];
    stream.read_exact(&mut len).await?;

    println!("{:?}", len);

    let mut checksum = vec![0; 4];
    stream.read_exact(&mut checksum).await?;

    println!("{:?}", checksum);

    let mut payload = vec![0; 102];
    stream.read_exact(&mut payload).await?;

    println!("{:?}", payload);

    println!("Finished reading version");
    Ok(())
}

async fn read_verack_message(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut verack = vec![0; 24];
    stream.read_exact(&mut verack).await.unwrap();

    println!("{:?}", verack);

    println!("Finished reading verack");
    Ok(())
}

// fn build_version_message_vec(address: SocketAddr) -> Vec<u8> {
//     vec![
//         // header
//         249, 190, 180, 217, // magic
//         118, 101, 114, 115, 105, 111, 110, 0, 0, 0, 0, 0, // "version" command
//         86, 0, 0, 0, // payload len
//         94, 38, 138, 233, // checksum

//         // payload
//         113, 17, 1, 0, // protocol version
//         0, 0, 0, 0, 0, 0, 0, 0, // services
//         0, 0, 0, 0, 0, 0, 0, 0, // time
//         // 36, 99, 31, 100, 0, 0, 0, 0, // time
//         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 134, 195, 185, 52, 32, 141, // recipient address info
//         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0, // sender address info
//         0, 0, 0, 0, 0, 0, 0, 0, // Node ID,
//         // 131, 67, 161, 229, 100, 242, 249, 181, // Node ID,
//         0, // "" sub-version string, 0 bytes long
//         0, // relay
//         0, 0, 0, 0, // last block sending node
//     ]
// }

// fn build_verack_message_vec() -> Vec<u8> {
//     vec![
//         // header
//         249, 190, 180, 217, // magic
//         18, 101, 114, 97, 99, 107, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 0, // payload len
//         93, 246, 224, 226, // checksum
//     ]
// }

// fn build_version_message(address: SocketAddr) -> message::NetworkMessage {
//     // Building version message, see https://en.bitcoin.it/wiki/Protocol_documentation#version
//     let my_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

//     // "bitfield of features to be enabled for this connection"
//     let services = constants::ServiceFlags::NONE;

//     // "standard UNIX timestamp in seconds"
//     // let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time error").as_secs();
//     let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time error").as_secs();

//     // "The network address of the node receiving this message"
//     let addr_recv = address::Address::new(&address, constants::ServiceFlags::NONE);

//     // "The network address of the node emitting this message"
//     let addr_from = address::Address::new(&my_address, constants::ServiceFlags::NONE);

//     // "Node random nonce, randomly generated every time a version packet is sent. This nonce is used to detect connections to self."
//     // let nonce: u64 = secp256k1::rand::thread_rng().gen();
//     let nonce: u64 = secp256k1::rand::thread_rng().gen();

//     // "User Agent (0x00 if string is 0 bytes long)"
//     let user_agent = String::from("rust-example");

//     // "The last block received by the emitting node"
//     let start_height: i32 = 0;

//     // Construct the message
//     message::NetworkMessage::Version(message_network::VersionMessage::new(
//         services,
//         timestamp as i64,
//         addr_recv,
//         addr_from,
//         nonce,
//         user_agent,
//         start_height,
//     ))
// }
