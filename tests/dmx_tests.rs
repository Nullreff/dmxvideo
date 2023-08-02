use artnet_protocol::*;
use dmxvideo::*;
use std::net::{UdpSocket, ToSocketAddrs};
use std::iter;

const TEST_ADDRESS : &str = "127.0.0.1";
const TEST_PORT : u16 = 6455;

#[test]
fn it_receives_a_dmx_universe() {
    let socket = UdpSocket::bind((TEST_ADDRESS, TEST_PORT)).unwrap();
    let broadcast_addr = ("127.0.0.1", 6454).to_socket_addrs().unwrap().next().unwrap();
    //socket.set_broadcast(true).unwrap();

    let buff = ArtCommand::Poll(Poll::default()).write_to_buffer().unwrap();
    socket.send_to(&buff, &broadcast_addr).unwrap();

    let mut buffer = [0u8; 1024];
    let (length, addr) = socket.recv_from(&mut buffer).unwrap();
    let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();
    
    println!("Received {:?}", command);
    match command {
        ArtCommand::Poll(poll) => {},
        ArtCommand::PollReply(reply) => {
            let command = ArtCommand::Output(Output {
                data: iter::repeat(255).take(UNIVERSE_SIZE).collect::<Vec<u8>>().into(),
                ..Output::default()
            });
            let bytes = command.write_to_buffer().unwrap();
            socket.send_to(&bytes, &addr).unwrap();
        },
        _ => {}
    }
}