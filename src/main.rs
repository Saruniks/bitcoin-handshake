use sha2::{Digest, Sha256};
use std::net::{IpAddr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

static VERACK_MESSAGE: &[u8] = &[
    // header
    249, 190, 180, 217, // magic
    18, 101, 114, 97, 99, 107, 0, 0, 0, 0, 0, 0, // "verack" command
    0, 0, 0, 0, // payload len
    93, 246, 224, 226, // checksum
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = "134.195.185.52:8333".parse()?;

    let mut stream = TcpStream::connect(address).await?;

    stream.write_all(build_version_message(address).as_slice()).await?;
    println!("Version message sent");
    println!("Waiting for response...");
    read_version_message(&mut stream).await?;
    println!("Version message received");

    stream.write_all(VERACK_MESSAGE).await?;
    println!("Verack message sent");
    println!("Waiting for response...");
    read_verack_message(&mut stream).await?;
    println!("Verack message received");

    println!("Handshake succeded");
    Ok(())
}

fn build_version_message(address: SocketAddr) -> Vec<u8> {
    let mut payload: Vec<u8> = vec![
        113, 17, 1, 0, // protocol version
        0, 0, 0, 0, 0, 0, 0, 0, // services
        0, 0, 0, 0, 0, 0, 0, 0, // time
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, // start of recipient address bytes
    ];

    let mut addr = socket_addr_to_vec(address);
    payload.append(&mut addr);
    payload.append(&mut vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0, // sender address info
        0, 0, 0, 0, 0, 0, 0, 0, // Node ID,
        0, // "" sub-version string, 0 bytes long
        0, // relay
        0, 0, 0, 0, // last block sending node
    ]);

    let mut header: Vec<u8> = vec![
        // header
        249, 190, 180, 217, // magic
        118, 101, 114, 115, 105, 111, 110, 0, 0, 0, 0, 0, // "version" command
    ];

    // Add payload len
    header.extend_from_slice(&(payload.len() as u32).to_le_bytes());

    // Add checksum
    let checksum = get_first_4_bytes_of_double_sha256(&payload);
    header.extend_from_slice(&checksum);
    header.extend(&payload);
    header
}

fn socket_addr_to_vec(socket_addr: SocketAddr) -> Vec<u8> {
    let ip = match socket_addr.ip() {
        IpAddr::V4(ip) => ip.octets().to_vec(),
        IpAddr::V6(ip) => ip.octets().to_vec(),
    };

    let port = socket_addr.port();
    let mut port_bytes = [0u8; 2];
    port_bytes.copy_from_slice(&port.to_be_bytes());

    [ip, port_bytes.to_vec()].concat()
}

async fn read_version_message(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut header = vec![0; 24];
    stream.read_exact(&mut header).await?;

    if "version" != std::str::from_utf8(&header[4..11])? {
        return Err("Unexpected command".into());
    }

    let len = u32::from_le_bytes([header[16], header[17], header[18], header[19]]);

    let mut payload = vec![0; len as usize];
    stream.read_exact(&mut payload).await?;

    let checksum = get_first_4_bytes_of_double_sha256(&payload);

    if checksum == header[20..24] {
        Ok(())
    } else {
        Err("Invalid version message checksum".into())
    }
}

async fn read_verack_message(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut verack = vec![0; 24];
    stream.read_exact(&mut verack).await?;

    if "verack" != std::str::from_utf8(&verack[4..10])? {
        return Err("Unexpected command".into());
    }

    if [93, 246, 224, 226] != verack[20..24] {
        return Err("Invalid verack message checksum".into());
    }

    Ok(())
}

fn get_first_4_bytes_of_double_sha256(data: &[u8]) -> [u8; 4] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut hasher2 = Sha256::new();
    hasher2.update(hash);
    let hash2 = hasher2.finalize();

    [hash2[0], hash2[1], hash2[2], hash2[3]]
}
