extern crate futures;
extern crate tokio;
extern crate tokio_util;

use std::net::SocketAddr;

use futures::StreamExt;

use self::tokio::net::UdpSocket;
use self::tokio_util::udp::UdpFramed;

use super::{AodvCodec, AODV_PORT};

pub async fn aodv() {
    // Bind to the AODV port
    let addr = SocketAddr::new("0.0.0.0".parse().unwrap(), AODV_PORT);
    let socket = UdpSocket::bind(&addr).await.unwrap();
    println!("Started listening on {}", AODV_PORT);

    let framed =  UdpFramed::new(socket, AodvCodec);

    framed.into_future().await;
    // let stream = stream
    //     .map_err(|err| eprintln!("{}", err)) // BUG: Crashes when malformed packet is sent
    //     .for_each(|(addr, msg)| {
    //         println!("{:?}", addr);
    //         println!("{:?}", msg);
    //         future::ok(())
    //     });

    // current_thread::run(|_| {
    //     current_thread::spawn(stream);
    // })
}
